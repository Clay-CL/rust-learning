# Building an CLI tool
- The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. 
- As a result, the Rust community has developed guidelines for splitting the separate concerns of a binary program when main starts getting large. This process has the following steps:
    * Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
    * As long as your command line parsing logic is small, it can remain in main.rs.
    * When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
- The responsibilities that remain in the main function after this process should be limited to the following:
    * Calling the command line parsing logic with the argument values
    * Setting up any other configuration
    * Calling a run function in lib.rs
    * Handling the error if run returns an error

