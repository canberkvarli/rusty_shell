# Rusty Shell

A **personal learning project** inspired by [joshmcguigan](https://github.com/joshmcguigan) to explore Rust’s systems programming features by building a minimal shell. It demonstrates:

- **Built-in commands** like `cd` (changes the current directory) and `exit` (quits the shell).  
- **Spawning external commands** (e.g., `ls`, `echo`) in child processes.  
- **Simple pipelining** (e.g., `ls | grep foo | wc -l`) by connecting the output of one command to the input of the next.

---

## Why Build It?

- **Hands-On Rust**: Practice ownership, borrowing, error handling, and Rust’s type system.  
- **Systems Programming**: Learn how to manage processes (`std::process::Command`), handle stdio streams, and create simple pipelines.  
- **Expandability**: A foundation for adding features like redirection, job control, or advanced parsing.

---

## Usage

1. **Clone and Build**

    ```bash
    git clone https://github.com/your-username/rusty_shell.git
    cd rusty_shell
    cargo build
    ```

2. **Run the Shell**

    ```bash
    cargo run
    ```

    You’ll see a prompt (`> `). Type in commands and press Enter.

3. **Example Commands**

    - **Built-Ins**  
      - `cd /path/to/dir`  
        - Changes current directory; defaults to `/` if none is provided.  
      - `exit`  
        - Quits the shell.

    - **Piped External Commands**  

      ```bash
      > ls | grep src | wc -l
      ```

    - **Regular External Commands**  

      ```bash
      > echo Hello Rust!
      Hello Rust!
      ```

---

## How It Works

1. **Prompt & Read**  
   Prints `> ` and waits for user input.

2. **Split on Pipes**  
   Splits the line on `|` to create a pipeline of commands.

3. **Built-In vs External**  
   - If the user types `cd` or `exit`, handle it inside the shell.  
   - Otherwise, spawn an external command.

4. **Chaining Commands**  
   Connect one command’s `stdout` to the next command’s `stdin`, with the last command’s output going to the terminal.

---

## Shout-Out

Huge thanks to [joshmcguigan](https://github.com/joshmcguigan/rust-shell) for the original tutorial and inspiration!

---

## Learning Points

- **Process Management**: Spawning processes, piping, and managing stdio with `Command::new(...)`.  
- **Iterators & String Slices**: Using Rust iterators to split and manage input strings.  
- **Error Handling**: Currently uses `.unwrap()`; can be improved with robust error messages.

---

## Future Extensions

- **Advanced Argument Parsing**: Handle quoted strings, globbing, environment variables, etc.  
- **Redirection**: Support `>` and `<` operators for file I/O redirection.  
- **Job Control**: Enable background processes (`&`) and job tracking.
