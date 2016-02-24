# simlink

Do you remember the order of args to make a symlink? I usually don't! 

Simlink is just a wrapper around `ln -s`, but it doesn't care which order you specify arguments! All you need is a file/directory that exists, and one that doesn't (yet!). This is going to [change the world](http://www.zombo.com/)!!

		
### Building

You'll need [Rust](https://www.rust-lang.org/) 1.6.0.

	cargo install simlink

or if you want to build it locally:

	cargo build
	./target/debug/simlink some_path some_other_path
	

### Demo

	# ~/.vimrc exists, the others do not.

	simlink ~/.foo ~/.vimrc
	simlink ~/.vimrc ~/.bar
	
### Are you serious?

- pants

--- 

© 2016 Dave Parfitt
