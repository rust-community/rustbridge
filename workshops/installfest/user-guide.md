# RustBridge InstallFest: User Guide

## Introduction

Welcome!
This tutorial walks you through the process of installing Rust on your computer and setting up a text editor to start programming with Rust.
You don't need to have any programming experience to use this tutorial.
However, an understanding of basic operations of your computer, such as downloading an app, is assumed.
The entire process should take you less than 30 minutes.

Before you begin, make sure you have a computer with:

* Access to the Internet
* An operating system that supports Rust, including macOS, Linux, or Windows.

Let's get started!

## Using the Terminal

A terminal is a text-based way of interacting with your computer without a graphical interface.
We're using a terminal because it's the standard method of installing and working with programming tools.
Your computer already has a terminal tool installed by default.
Review the instructions for the operating system of your choice.

### macOS

1.  Open Terminal by either selecting the Spotlight menu (magnifying glass in the upper right) or by pressing the command and space keys together, then typing "Terminal".)

    When you first open Terminal, you'll see a prompt.
    You type commands to the computer after the prompt.

    It'll look something like this:

    ```
    Last login: Some Date on console
    Janes-MacBookPro:~ jane$
    ```

### Windows

On windows the terminal is called a command prompt. You can access it by opening
the start menu and typing `cmd.exe` or `command prompt`. Windows should find it
for you.

After starting the command prompt, you can see a black screen saying something
like this:

    C:\Users\yourname>

Try typing commands like `dir`.

Well done!


### Linux

1.  Ensure `curl` is installed.

    ```
    which curl
    ```

    Installation is successful if you see any output.
    It will look something like this:

    ```
    /usr/bin/curl
    ```

    Curl is used to download files from the Internet.
    We need to make sure it's installed on your computer before attempting to download Rust files.

    If there is no output (the prompt simply appears again), or you get a message saying something like "curl not found" you must install curl.
    Use your system's package manager to install it.

    On a Debian-based system like Ubuntu, this would be:

    ```
    apt-get install curl
    ```

## Installing Rust

The next step after opening the terminal is installing rustup, which installs Rust and Cargo, which is a package management (installing software written in Rust) and build tool (converting code into a usable program).

### macOS

1.  In your web browser, navigate to [rustup.rs](https://rustup.rs).

2.  To download and run a program that installs the correct version of rustup for your computer, copy and paste the text on the webpage into your terminal.
It should look something like this:

    ```
    Janes-MacBookPro:~ curl https://sh.rustup.rs -sSf | sh
    ```

3.  Press return.

    The output describes how Rust and Cargo will be installed on your computer.

4.   After you've read through the output, press return to accept the default installation settings.
    It may take up to a couple minutes to install.
    Once it's done installing, you'll see an installation is complete message.

5.  Follow the instructions to configure the terminal.
    Copy and paste the text.
    It'll look something like this:

    ```
    Janes-MacBookPro:~ source $HOME/.cargo/env
    ```

    There won't be any output or confirmation. You'll just see the prompt again.

6.  To confirm that cargo installed correctly, type `cargo` at the prompt and press return.
    If it installed correctly, you'll see an output with help content.


### Windows

1. Go to the [Rust website](https://www.rust-lang.org).

2. Click on *Install* to download the Rust installer.

3. Open the file `rust-1.11.0-i686-pc-windows-gnu.msi` (or similar) you just
   downloaded.s

4. Now the installation dialog starts. Confirm any warnings and stick to the
   default options when prompted.

Now you should have Rust installed on your machine.

To verify that, open a *new* command prompt like above. Don't use one that has
already been open, because it does not know about Rust yet.

Type in the command `rustc --version`. The Rust compiler will tell you its
current version:

    C:\Users\yourname>rustc --version
    rustc 1.11.0 (9b21dcd6a 2016-08-15)

    C:\Users\yourname>

Now you are ready to go!

You can create a new Rust project using `cargo`, Rust's package manager, like
this:

    C:\Users\yourname>cargo new my-rust-project --bin

    C:\Users\yourname>cd my-rust-project

    C:\Users\yourname\my-rust-project>cargo run
       Compiling my-rust-project v0.1.0 (file:///C:/Users/yourname/my-rust-project)
         Running `target\debug\my-rust-project.exe`
    Hello, world!

    C:\Users\yourname\my-rust-project>

Well done! You just compiled and executed your first Rust program.


## Setting up a text editor

A text editor (also commonly just called an editor) is a program used to edit text files.
Examples of common editors used to program are Atom, Sublime Text, Microsoft Visual Studio Code, Vim, and Emacs.

Word processors, like Microsoft Word, aren't the same as text editors and aren't designed for editing source code.

If you already installed and use a text editor, skip to the "Installing the Rust package for your editor" section.
If you don't have a text editor or are not sure if you have one, we recommend installing Atom.
The next section walks you through the installation.

### Installing Atom

#### MacOSX

1.  In your web browser, navigate to [Atom website](https://atom.io).

2.  Click the Download button to download a zip file.

3.  Browse to the zip file in your Downloads directory or click the file the your browser's download bar.

4.  Extract the zip file by double clicking the .zip file.

5.  Move the Atom.app file to your Applications directory.
    You can drag and drop the file.

6.  In the Applications directory, open the Atom app by double clicking.

    If you get a security message about opening an application downloaded from the Internet, it's okay to proceed. Click Open.

    When Atom opens, you'll see a Welcome and Welcome Guide pages as tabs.

#### Windows

In your web browser, navigate to the [Atom website](https://atom.io). Hit
*Download Windows Installer* to download `AtomSetup.exe`. Open it to install
Atom.

Follow the installation dialog. At the end, Atom should start. You can always
access it using the start menu.

### Installing the Rust package for your text editor

The final step is installing a Rust language package for a text editor.
The package makes it easier to code with Rust with features like colored text and auto-formatting.

1.  In Atom, navigate to Preferences, then Install.
    (On Windows, you can find this under File, then Settings; or hit Ctrl+,)

2.  Search for "rust".

3.  Select the "language-rust" package by the user "zargony".
    Click Install.

    Wait for the package to download and install.

4.  Still in Atom, click on the Packages tab.
    Under the Community Packages section, verify that "language-rust" is listed.

5.  Open a new text file by selecting New File under File.
    You may already have an untitled file already open.

6.  In the bottom right, click Plain Text to change the file type.

7.  Search for "rust".
    Click "Rust" in the search results.
    "Rust" should replace "Plain Text" now.

8.  To test the language package, type the following:

    ```
    fn main() {}
    ```

    Notice that "fn" and "main" appear in different colors.

## Conclusion

You've now installed Rust and optimized your text editor for Rust.
You're ready to start learning Rust!
