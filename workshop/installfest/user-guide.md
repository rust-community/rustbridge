# RustBridge InstallFest: User Guide

* Introduction
  * Goals of InstallFest / what you will achieve by the end of this process
    * You have Rust and Cargo installed on your computer and a text editor ready to create Rust programs.
  * Prerequisites / assumed knowledge
    * You have a modern computer capable of connecting to the Internet.
    * You are running an operating system that Rust supports:
      * macOS
      * Linux
      * Windows
    * You don't need any experience programming at all.
      You do need to be familiar with basic operation of your computer.
      You must have administrator access to your computer (the ability to install files to system-wide directories.)
* Using the terminal
  * What is a terminal? Why are we using it?
  * macOS and Linux have terminals by default. Does Windows?
  * Opening the terminal
    * macOS: Select from Spotlight menu (click on magnifying glass or hit command-space)
  * Explain prompt and command entering
  * Verify curl is installed: `which curl`
    * If you get output that looks like "/usr/bin/curl", it is installed. The text it prints is where the program is stored on your system.
    * If there is no output (the prompt simply appears again), or you get a message saying something like "curl not found" you must install curl.
      * How do you install it?
* Installing Rust
  * Installing rustup, rustc, and cargo
    * Explain what rustup is. Why use it instead of installing Rust directly?
    * User goes to rustup.rs website.
    * Explain what the command is going to do.
    * Copy/paste paste the command into your terminal and press return.
    * rustup will display a bunch of info about what it's going to do.
      You should read this, but don't worry if you don't understand it all.
      Press the return/enter key to proceed with the default installation, which will install the latest stable version of Rust.
    * It will take anywhere from a few seconds to a few minutes to download Rust depending on your Internet connection's speed.
    * Eventually, you will see a message saying Rust is installed.
    * Follow the message about configuring your current shell (terminal) by running `source $HOME/.cargo/env`. There will not be any output, you'll just see the prompt again.
    * Run `cargo` and `rustc` (notice the "c" on the end). If you see help output for each of these programs, they are installed!
    * Very briefly explain why it's `rustc` and not `rust`.
* Installing an editor
  * Choosing a text editor
    * A text editor (also commonly just called "an editor") is a program used to change the content of files containing text. Examples of common editors are Atom, Sublime Text, Microsoft Visual Studio Code, Vim, and Emacs.
    * Word processors are not the same as text editors. Word processors like Microsoft Word are intended for other types of documents and not program source code.
    * If you already have an editor installed that you prefer, use that.
    * If you don't have an editor installed or you're not sure if you have one installed, we recommend installing Atom.
  * Installing Atom
    * Go to [Atom website](https://atom.io).
    * Click the big download button.
    * Find the download in your downloads directory or click the file in your browser's download section.
    * Extract the zip archive by double clicking it.
    * Move Atom.app to your applications directory. (Drag and drop)
    * Go to your applications directory and double click the Atom app.
    * If you get a security message about having downloaded it from the Internet, it is okay. Click "Open".
    * The first time you run Atom you will get a welcome page and guide.
      You may browse this guide at your leisure if you wish.
  * Installing the Rust package for your editor
    * Explain a package is in the context of the editor. (Differentiate from "rust/cargo package" and packages in general.)
    * Explain why we want the Rust package.
    * Atom > Preferences > Install > search for "Rust".
    * Select the package "language-rust" by the user "zargony" and click install.
    * Wait for the package to download and install.
    * Click on the "Packages" tab and you should see the language-rust package you just installed under the "Community Packages" section.
  * Trying out the editor
    * Open a new text file (File > New File) if you don't already have an "Untitled" document open.
    * In the bottom right corner of the window, click "Plain Text" to change the type of file you're editing.
    * Search for "rust" and then click on "Rust" in the search results.
    * The text in the bottom right corner of the window should now say "Rust" instead of "Plain Text".
    * To test that Rust package is working, type: `fn main() {}` in the window. You should see "fn" and "main" appear in different colors.
* Conclusion
  * Review what you've done
  * Reiterate what you can now do
