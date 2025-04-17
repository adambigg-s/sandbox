# sandbox

my take on the classic "falling sand" style of game, built in Rust with a soft pastel palette
currently, there is not many elements. i am trying to just get an efficient architecture first

this project uses some hackish things to get parallel processing to work as efficiently as possible, it runs super fast but has some data race issues

the goal is to be able to simulate a 3440x2560 world at 100 fps. i would like for every pixel of my 4k monitor to be simulated, i think this would look really cool
currently, this program can run on my pretty dated hardware at 2560x1080 extremely easily at 60 fps.

https://www.pcg-random.org/

![alt text](https://github.com/adambigg-s/sandbox/blob/main/demo/falling_sand.gif)

example on my large monitor
![alt text](https://github.com/adambigg-s/sandbox/blob/main/demo/2560x1440.png)
