	// thread launching functions
	pub fn spawn_server( &mut self){
		let port = self.port;
		let motedb_arc = self.motedb.clone();
		let remotedb_arc = self.remotedb.clone();
		let mut thread_guards = self.thread_guards.lock().unwrap();
		thread_guards.push(
			Thread::spawn( move ||
				Hub::server( port, motedb_arc, remotedb_arc)));
		drop( thread_guards);
		println!( "server proc spawned.");}

	pub fn spawn_bootstrap( &mut self){
		let remotedb_arc = self.remotedb.clone();
		let mut thread_guards = self.thread_guards.lock().unwrap();
		thread_guards.push(
			Thread::spawn( move ||
				Hub::bootstrap( remotedb_arc)));
		drop( thread_guards);
		println!( "bootstrap proc spawned.");}

	pub fn spawn_push( &mut self){
		let motedb_arc = self.motedb.clone();
		let remotedb_arc = self.remotedb.clone();
		let mut thread_guards = self.thread_guards.lock().unwrap();
		thread_guards.push(
			Thread::spawn( move ||
				Hub::push( motedb_arc, remotedb_arc)));
		drop( thread_guards);
		println!( "push proc spawned.");}

	pub fn spawn_greet( &mut self){
		let hostname = self.hostname.clone();
		let port = self.port;
		let remotedb_arc = self.remotedb.clone();
		let mut thread_guards = self.thread_guards.lock().unwrap();
		thread_guards.push(
			Thread::spawn( move ||
				Hub::greet( hostname, port, remotedb_arc)));
		drop( thread_guards);
		println!( "greet proc spawned.");}

	// thread functions

	fn server( port: u16, motedb_arc: Arc<Mutex<Vec<Mote>>>,
			remotedb_arc: Arc<Mutex<Vec<RemoteHub>>>){
		// open server
		let mut listener = TcpListener::bind(
			( "localhost", port)).unwrap();
		println!( "[sv] opened listener on: {}",
			listener.socket_name().unwrap());
		let mut acceptor = listener.listen();
		// wait for incoming streams
		for stream in acceptor.incoming() {
			if let Ok( client) = stream {
				// clone motedb and remotedb handles
				let motedb_arc = motedb_arc.clone();
				let remotedb_arc = remotedb_arc.clone();
				// spawn client handler
				Thread::spawn( move ||
					Hub::serve( client, motedb_arc, remotedb_arc)).detach()}}
		// close server
		println!( "[sv] closing listener");
		drop( acceptor);}

	fn serve( mut client_stream: TcpStream, motedb_arc: Arc<Mutex<Vec<Mote>>>,
			remotedb_arc: Arc<Mutex<Vec<RemoteHub>>>){
		let client_addr = client_stream.peer_name().unwrap();
		//println!( "[sv] client {} connected", client_addr);

		// start reading commands from client
		let mut reader = BufferedReader::new( client_stream.clone());
		while let Ok( line) = reader.read_line() {
			// parse command
			let line_trimmed = line.as_slice().trim();
			let command = Command::from_str( line_trimmed);
			if command.is_none() {
				println!( "[sv] failed to parse command from {}: {}",
					client_addr, line_trimmed);
				continue;}
			let command = command.unwrap();

			// handle command
			let response : Response = match command {
				// todo: adjust hello command to contain remote addr
				Hello( hostname) => Hub::greet_remote( &remotedb_arc, hostname),
				// request all the remotes we know about
				OthersReq => Hub::get_others( &remotedb_arc),
				// record that this remote has the specified mote
				HaveDec( _hash) => Deny,
				// reply with whether we have the specified mote
				HaveReq( hash) => Hub::have_mote( &motedb_arc, hash),
				// return the given mote ( if we have it )
				Get( hash) => Hub::get_mote( &motedb_arc, hash),
				// return whether we want the specified mote
				WantReq( hash) => Hub::want_mote( &motedb_arc, hash),
				// accept the given mote into our db
				Take( json) => Hub::take_mote( &motedb_arc, json),
			};

			client_stream.write_line(
				response.to_string().as_slice()).ok();}

		// clean up
		//println!( "[sv] client {} disconnected", client_addr);
		client_stream.close_write().ok();}

	fn bootstrap( remotedb_arc: Arc<Mutex<Vec<RemoteHub>>>){
		let others_req_msg = OthersReq.to_string();
		loop {
			// copy addresses from current remotedb
			let remotedb = remotedb_arc.lock().unwrap();
			let mut remotes_addr : Vec<SocketAddr> = Vec::new();
			for ref remote in remotedb.iter() {
				remotes_addr.push( remote.addr.clone());}
			drop( remotedb);

			// get new remotes from existing remotes
			let mut new_remotes : Vec<SocketAddr> = Vec::new();
			for &addr in remotes_addr.iter() {
				// connect to remote
				//println!( "[bs] attempting to bootstrap against: {}", addr);
				let remote_stream =
					TcpStream::connect_timeout(
						addr.clone(), Duration::seconds( 10));
				if let Err( _msg) = remote_stream {
					//println!( "[bs] failed to connect to {}: {}", addr, _msg);
					continue;}
				let mut remote_stream = remote_stream.unwrap();

				// send request
				remote_stream.write_line( others_req_msg.as_slice()).ok();
				remote_stream.close_write().ok();

				// parse response
				let mut reader = BufferedReader::new( remote_stream.clone());
				let line = reader.read_line().ok();
				remote_stream.close_read().ok();
				if line.is_none() {
					println!( "[bs] failed to read response from {}", addr);
					continue;}
				let line = line.unwrap();
				let line_trimmed = line.as_slice().trim();
				let response = Response::from_str( line_trimmed);
				if response.is_none() {
					println!( "[bs] failed to parse response from {}: {}",
						addr, line_trimmed);
					continue;}
				let response = response.unwrap();

				// handle response
				match response {
					OkayResult( Json::Array( list)) => {
						//println!( "[bs] result response from {}: {}", addr, list);
						for ref entry in list.iter() {
							match *entry {
								&Json::String( ref string) => {
									let new_addr : Option<SocketAddr> =
										string.as_slice().to_socket_addr().ok();
									if new_addr.is_some(){
										new_remotes.push( new_addr.unwrap());}}
								_ => ()}}}
					Deny => {
						println!( "[bs] deny response from {}", addr);}
					bad => {
						println!( "[bs] bad response from {}: {}",
							addr, bad);}}

				// move on to next remote
				continue;}

			//write new remotes to db
			let mut remotedb = remotedb_arc.lock().unwrap();
			//let size = remotedb.len();
			for &new_addr in new_remotes.iter() {
				let new_remote = RemoteHub::new( new_addr.clone());
				if ! remotedb.as_slice().contains( &new_remote) {
					println!( "[bs] adding new remote: {}", new_addr);
					remotedb.push( new_remote);}}
			//if size != remotedb.len() {
			//	println!( "[bs] remote list updated: {}", remotedb.as_slice());}
			drop( remotedb);

			//wait for a while before polling again
			sleep( Duration::milliseconds( BOOTSTRAP_PAUSE_MILLIS));}}

	fn push( motedb_arc: Arc<Mutex<Vec<Mote>>>,
			remotedb_arc: Arc<Mutex<Vec<RemoteHub>>>){
		loop {
			// copy addresses from current remotedb
			let remotedb = remotedb_arc.lock().unwrap();
			let mut remotes_addr : Vec<SocketAddr> = Vec::new();
			for ref remote in remotedb.iter() {
				remotes_addr.push( remote.addr.clone());}
			drop( remotedb);
			//println!( "[ps] remote addrs: {}", remotes_addr);

			// copy motedb from current motedb
			let motedb = motedb_arc.lock().unwrap();
			let motedb_copy = motedb.clone();
			drop( motedb);
			let motedb = motedb_copy;

			// push 
			for &remote_addr in remotes_addr.iter() {
				// connect to remote
				//println!( "[ps] attempting to push to: {}", remote_addr);
				let remote_stream =
					TcpStream::connect_timeout(
						remote_addr.clone(), Duration::seconds( 10));
				if let Err( _msg) = remote_stream {
					//println!( "[ps] failed to connect to {}: {}",
					//	remote_addr, _msg);
					continue;}
				let mut remote_stream = remote_stream.unwrap();
				let mut reader = BufferedReader::new( remote_stream.clone());

				for mote in motedb.iter() {
					let mote_hash = hash::hash( mote);
					//println!( "[ps] offering {} mote {:016x}",
					//	remote_addr, mote_hash);

					// send want? request
					let want_req_msg = WantReq( mote_hash).to_string();
					remote_stream.write_line( want_req_msg.as_slice()).ok();

					// parse want? response
					let line = reader.read_line().ok();
					if line.is_none() {
						println!( "[ps] failed to read response from {}", remote_addr);
						continue;}
					let line = line.unwrap();
					let line_trimmed = line.as_slice().trim();
					let response = Response::from_str( line_trimmed);
					if response.is_none() {
						println!( "[ps] failed to parse response from {}: {}",
							remote_addr, line_trimmed);
						continue;}
					let response = response.unwrap();

					// handle want? response
					match response {
						Affirm => (),
						Deny => {
							if PUSH_LOG_DECLINE {
								println!( "[ps] remote {} declined mote {:016x}",
									remote_addr, mote_hash);}
							continue;}
						bad => {
							println!( "[ps] bad want response from {}: {}", remote_addr, bad);
							continue;}}

					//send take request
					let take_req_msg = Take(
						mote.to_msg().to_json()).to_string();
					remote_stream.write_line( take_req_msg.as_slice()).ok();

					// parse take response
					let line = reader.read_line().ok();
					if line.is_none() {
						println!( "[ps] failed to read response from {}", remote_addr);
						continue;}
					let line = line.unwrap();
					let line_trimmed = line.as_slice().trim();
					let response = Response::from_str( line_trimmed);
					if response.is_none() {
						println!( "[ps] failed to parse response from {}: {}",
							remote_addr, line_trimmed);
						continue;}
					let response = response.unwrap();

					// handle take response
					match response {
						Okay => {
							println!( "[ps] remote {} accepted mote {:016x}",
								remote_addr, mote_hash);}
						Deny => {
							println!( "[ps] remote {} denied mote {:016x}",
								remote_addr, mote_hash);}
						bad => {
							println!( "[ps] bad take response from {}: {}",
								remote_addr, bad);}}

					//move on to next mote
					continue;}

				// move on to next remote
				drop( remote_stream);
				continue;}

			//wait for a while until polling
			sleep( Duration::milliseconds( PUSH_PAUSE_MILLIS));}}

	fn greet( hostname: String, port: u16,
			remotedb_arc: Arc<Mutex<Vec<RemoteHub>>>){
		let hello_msg = Hello(
			format!( "{}:{}", hostname, port)).to_string();
		loop {
			// copy addresses from current remotedb
			let remotedb = remotedb_arc.lock().unwrap();
			let mut remotes_addr : Vec<SocketAddr> = Vec::new();
			for ref remote in remotedb.iter() {
				remotes_addr.push( remote.addr.clone());}
			drop( remotedb);

			// push 
			for &remote_addr in remotes_addr.iter() {
				// connect to remote
				//println!( "[gt] greeting {}", remote_addr);
				let remote_stream =
					TcpStream::connect_timeout(
						remote_addr.clone(), Duration::seconds( 10));
				if let Err( _msg) = remote_stream {
					//println!( "[gt] failed to connect to {}: {}",
					//	remote_addr, _msg);
					continue;}
				let mut remote_stream = remote_stream.unwrap();
				let mut reader = BufferedReader::new( remote_stream.clone());

				// send hello request
				remote_stream.write_line( hello_msg.as_slice()).ok();

				// parse hello response
				let line = reader.read_line().ok();
				if line.is_none() {
					println!( "[gt] failed to read response from {}", remote_addr);
					continue;}
				let line = line.unwrap();
				let line_trimmed = line.as_slice().trim();
				let response = Response::from_str( line_trimmed);
				if response.is_none() {
					println!( "[gt] failed to parse response from {}: {}",
						remote_addr, line_trimmed);
					continue;}
				let response = response.unwrap();

				// handle hello response
				match response {
					Okay => (),
					Deny => {
						println!( "[gt] remote {} denied greeting", remote_addr);
						continue;}
					bad => {
						println!( "[gt] bad greet response from {}: {}",
							remote_addr, bad);
						continue;}}

				// move on to next remote
				drop( remote_stream);
				continue;}

			//wait for a while until polling
			sleep( Duration::milliseconds( GREET_PAUSE_MILLIS));}}

	// command handling functions

	fn greet_remote( remotedb_arc: &Arc<Mutex<Vec<RemoteHub>>>,
			hostname: String) -> Response {
		let addr = hostname.as_slice().to_socket_addr().ok();
		if addr.is_none() { return Error;}
		let addr = addr.unwrap();
		let new_remote = RemoteHub::new( addr);
		let mut remotedb = remotedb_arc.lock().unwrap();
		if ! remotedb.as_slice().contains( &new_remote) {
			println!("[sv] greeted remote: {}", addr);
			remotedb.push( new_remote);}
		drop( remotedb);
		Okay}

	fn get_others(
			remotedb_arc: &Arc<Mutex<Vec<RemoteHub>>>) -> Response {
		let remotedb = remotedb_arc.lock().unwrap();
		let mut list : Vec<Json> = Vec::new();
		// push the addr of every remote we know of
		for remote in remotedb.iter() {
			list.push( Json::String( remote.addr.to_string()))}
		drop( remotedb);
		OkayResult( Json::Array( list))}

	fn have_mote(
			motedb_arc: &Arc<Mutex<Vec<Mote>>>, hash: u64) -> Response {
		let motedb = motedb_arc.lock().unwrap();
		let mut response = Deny;
		for mote in motedb.iter() {
			if hash == hash::hash( mote) {
				response = Affirm;
				break;}}
		drop( motedb);
		response}

	fn get_mote(
			motedb_arc: &Arc<Mutex<Vec<Mote>>>, hash: u64) -> Response {
		let motedb = motedb_arc.lock().unwrap();
		let mut response = Deny;
		for mote in motedb.iter() {
			if hash == hash::hash( mote) {
				let mote_json = mote.to_msg().to_json();
				response = OkayResult( mote_json);
				break;}}
		drop( motedb);
		response}

	fn want_mote(
			motedb_arc: &Arc<Mutex<Vec<Mote>>>, hash: u64) -> Response {
		let motedb = motedb_arc.lock().unwrap();
		let mut response = Affirm;
		for mote in motedb.iter() {
			if hash == hash::hash( mote) {
				response = Deny;
				break;}}
		drop( motedb);
		response}

	fn take_mote(
			motedb_arc: &Arc<Mutex<Vec<Mote>>>, json: Json) -> Response {
		// decode mote
		let mut decoder = json::Decoder::new( json);
		let mote_msg : Option<MoteMsg> =
			Decodable::decode( &mut decoder).ok();
		if mote_msg.is_none() { return Error;}
		let mote = Mote::from_msg( &mote_msg.unwrap());
		if mote.is_none() { return Error;}
		let mote = mote.unwrap();
		let mote_hash = hash::hash( &mote);
		println!("[sv] received new mote: {:016x} :: {}", mote_hash, mote);

		let mut motedb = motedb_arc.lock().unwrap();
		motedb.push( mote);
		drop( motedb);
		Okay}
