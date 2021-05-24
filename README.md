# gitstatus-rs
Uber simplified version of gitstatus used in Powerlevel10k

#### Why if gitstatus already exists
I wanted to use gitstatus on my OpenBSD system, but it seems to not support OpenBSD by default. 
I also tried compiling using the provided build script, but it failed. Then, I decided to build an
simple script using the git command, which was really slow and irritating.
Finally, I made this simple Rust "script" using libgit2 which is like 10 times the speed.
Ultimately, this is a possible alternative for users of operating systems that aren't supported 
by gitstatus, but have rust ports, which is seemingly very few. 

#### Installation
First off clone this repo,  
```
git clone https://github.com/p0l1t1c1an/gitstatus-rs.git
```
And then run cargo install
```
cargo install --path /path/to/gitstatus-rs
```

That's it!
Now a gitstatus binary should be install in your `$CARGO_HOME/bin`

#### Adding to Powerlevel10k

First, you need to make prompt_gitstatus function in your .p10k.zsh.
For example, this is mine:
```
  function prompt_gitstatus() {
    out=$($HOME/.cargo/bin/gitstatus)         
    if [ ! -z $out ];      
      then p10k segment -f 208 -i 'git' -t "$out"
    fi
  }
```

You may need to change the path to the executable and may want to change the -f and -i argument in p10k segment.  
-f changes the forground/text color to be printed out (208 is one of many oranges)  
-i is the icon for to be printed before the text (I put in git as a placeholder as I actually use a nerd fonts git icon)  

Next, you will need to add `gitstatus` to your left or right elements in .p10k.zsh.   
For example, this is mine:
```
  typeset -g POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(
    # =========================[ Line #1 ]=========================
    dir                     # current directory
    gitstatus               # git status
    # =========================[ Line #2 ]=========================
    newline                 # \n
    prompt_char             # prompt symbol
  )
```

#### Missing from original gitstatus
##### Left and Right Arrows
The Left and right arrow symbols are missing which represent behind and ahead the push remote, respectively.  
I'm actually not sure what this means in comparison to the down and up arrows meaning behind and ahead the remote.
(What is the push remote vs the remote?)

##### Merge Status
The original gitstatus shows you if you are in the middle of merging branches by printing out 'merge'.  
I will probably add this at some point. And when this is up it will check for conflicts, otherwise no conflicts will be shown.   
(Basically should show stashes, stages, unstaged, and untracked or merge state and conflicts)

##### Customizability
Basically, you could tell gitstatus ordering of the multiple statuses, colors for them, and I think custom icons for each.
I'm basically not going to add that as that is just way too much for something I think of as kinda useless.

#### Acknowledgements
[gitstatus](https://github.com/romkatv/gitstatus) - The original gitstatus that I based the output of this on (and the stashes count code on)  
[powerlevel10k](https://github.com/romkatv/powerlevel10k) - What this is meant to be used for


