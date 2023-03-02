use std::{
	io::{Error, Result},
	os::unix::process::CommandExt,
	process::Command,
};

use crate::{builder::CommandGroupBuilder, CommandGroup, GroupChild};
use nix::unistd::setsid;

impl CommandGroup for Command {
	fn group_spawn(&mut self) -> Result<GroupChild> {
		unsafe {
			self.pre_exec(|| setsid().map_err(Error::from).map(|_| ()));
		}

		self.spawn().map(GroupChild::new)
	}

	fn group(&mut self) -> CommandGroupBuilder<Command> {
		CommandGroupBuilder::new(self)
	}
}

impl CommandGroupBuilder<'_, Command> {
	/// Executes the command as a child process group, returning a handle to it.
	///
	/// By default, stdin, stdout and stderr are inherited from the parent.
	///
	/// On Windows, this creates a job object instead of a POSIX process group.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```no_run
	/// use std::process::Command;
	/// use command_group::CommandGroup;
	///
	/// Command::new("ls")
	///         .group()
	/// 		.spawn()
	///         .expect("ls command failed to start");
	/// ```
	pub fn spawn(&mut self) -> std::io::Result<GroupChild> {
		self.command.group_spawn()
	}
}
