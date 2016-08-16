# RustBridge InstallFest: Presenter Notes

Each bullet point here should the text of a different slide.
Nested bullet points are notes about other things that should appear on that slide.
Nested bullet points beginning with "PN:" are presenter notes for that slide.
??? is content that still needs to be filled in.

* RustBridge InstallFest
  * Main title card
* Goals
  * PN: The goal of InstallFest is to get your computer set up with Rust, Cargo, and a text editor you can use to work on Rust code.
* Prerequisites
  * PN: Explain what is expected of the user before beginning:
    working macOS, Windows, or Linux computer with Internet access.
    Prior programming experience is NOT expected, but may be helpful.
* Using the terminal
  * PN: Explain what a terminal is and why we are using it.
* macOS: Terminal.app
  * Screenshot of Terminal.app being selected in the Finder and/or Spotlight.
  * PN: Explain how to find and launch Terminal.app.
* Windows: ???
  * Screenshot of how to find and open whatever terminal app we recommend
  * PN: Explain how to find and launch the terminal app
* Linux: ???
  * Screenshot of how to find and open whatever terminal app we recommend
  * PN: Explain how to find and launch the terminal app
* Understanding the command line
  * PN: Explain what the command line is:
    A prompt is shown, the user enters a command as input, and the computer executes the command, optionally displaying some output in response.
* (Screenshot of an average looking command prompt)
* `$ echo hello`
  * PN: Have everyone run this command as a way of demonstrating command input and execution.
* `curl`
  * PN: Explain what curl is and that we will use it to install rustup.
  * PN: curl is installed with the system on macOS, but Windows/Linux users may need to install it. (How?)
* rustup
  * PN: Explain what rustup is:
    The official tool for installing Rust and managing Rust updates.
* rustup.rs
  * PN: Tell everyone to go to the website and look at the command it shows.
  * PN: Explain "WARNING: This is beta software" if it makes people nervous.
* `curl https://sh.rustup.rs -sSf | sh`
  * PN: Explain what the command is going to do, why `curl | sh` is potentially dangerous, but why we're doing it anyway in this case.
  * PN: Ask everyone to run the command.
* (Screenshot of what you see when you run the rustup installer)
  * PN: Tell everyone to accept the defaults and let rustup continuing installing.
* (Screenshot of successful rustup installation message)
* `source $home/.cargo/env`
  * PN: Explain that your terminal needs to be "reloaded" for it to "see" the new Rust installation, but that this only happens once.
* Verifying the installation
* `rustc`
  * PN: Everyone should see help output, which verifies rustc is installed.
  * PN: Explain that `rustc` is the program that compiles Rust source code, and that is what the "c" at the end of the name is for.
  * PN: Explain that generally everyone will not use `rustc` directly, but that it's good to know it's there.
* `cargo`
  * PN: Everyone should see help output, which verifies cargo is installed.
  * PN: Explain that cargo is a package management and build tool for Rust and your primary tool when making Rust programs.
  * PN: Explain package management as the ability to install other Rust software to use in your program.
  * PN: Explain build tool as a program to compile your program and dependent programs in an easy way, and that using rustc directly is "lower level" and less friendly.
