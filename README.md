# RustOSLife
My path towards learning Rust and Operating Systems

## What is it
My goal is to create a basic operating system, and to be able to run a Game of Life on it with some user inputs at the start of the program.

To reach this goal, I'll first need to learn some Rust fundamentals, and then I'll move on to creating an OS which targets a Mac Mini from early 2009.

This is not an attemp to create a fully functionnal OS, as I will not implement user space capabilities. Well, at least, not right now... >:)

## Context
This project is a self proposed project, based on my own interests, that I'll be doing as part of Innovation track of the Epitech School.

## Roadmap

- Release 1.0: OS with text display defined at compile time
    - Creation of a boot system or multiboot compliant system
    - Stack managment capabilities
    - Console managment and text display functions
<br>

- Release 2.0: Hardware interrupts and vectorial display
    - Apic / Timer implementation
    - Hardware interruptions, with AZERTY and QWERTY keyboard options
    - Virtual memory
    - Vectorial display library
<br>

- Release 3.0: Block device and file system
    - Block devices handling
    - FAT filesystem implementation
    - Creation of a kernel module system
<br>
- Release 4.0: Game of life as a kernel module

## Resources

During this attempt, I will heavily follow the amazing tutorial at [os.phil-opp.com](https://os.phil-opp.com/) <br>
I will also get a lot of information on [wiki.osdev.org](https://wiki.osdev.org/Expanded_Main_Page)<br>