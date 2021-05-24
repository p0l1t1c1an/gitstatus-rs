# gitstatus-rs
Uber simplified version of gitstatus used in Powerlevel10k

#### Why if gitstatus already exists
I wanted to use gitstatus on my OpenBSD system, but it seems to not support OpenBSD by default.  
Then, I decided to build a simple script using the git command, which was really slow and irritating.  
Finally, I made this simple Rust "script" using libgit2 which is like 10 times the speed of that script.  
Ultimately, this is a possible alternative for users that aren't supported, which is seemingly very few. 

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
Now a gitstatus binary should be install in your `$CARGO_HOME/bin` or `$HOME/.cargo/bin`

#### Adding to Powerlevel10k

First, you need to make prompt_gitstatus function in your .p10k.zsh.
For example, this is mine:
```
  function prompt_gitstatus() {
    out=$(path_to_cargo_home/bin/gitstatus)         
    if [ ! -z $out ];      
      then p10k segment -f 208 -i 'git' -t "$out"
    fi
  }
```

You will want to change the executable path and the -f and -i argument in p10k segment.  
-f changes the forground/text color to be printed out (208 is one of many oranges)  
-i is the icon for to be printed before the text (I put in git as a placeholder)  

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

#### Difference from original gitstatus
##### Up and Down Arrows
I change the up/down arrows to & and | these mean ahead/behind the remote.  
Not sure why its just what I chose. I didn't really like the arrows.

##### Left and Right Arrows
The left/right arrow symbols are missing which represent behind/ahead the push remote.  
I don't know what this means compared to the down/up arrows meaning behind/ahead the remote.  
(What is the push remote vs the remote?)

##### Merge Status
The original gitstatus prints out 'merge' if you are merging branches.  
I will probably add this at some point and only check for conflicts when in merge state.  
(Basically should show stashes, stages, unstaged, and untracked or merge state and conflicts)

##### Customizability
Basically, you could tell gitstatus the ordering, color, and icon for each status alone.  
I'm not going to add this as it is way too much for something I think of as kinda useless.

#### Acknowledgements
[gitstatus](https://github.com/romkatv/gitstatus) - The original gitstatus that I based the output of this on (and the stashes count code on)  
[powerlevel10k](https://github.com/romkatv/powerlevel10k) - What this is meant to be used for


