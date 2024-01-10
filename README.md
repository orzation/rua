# rua
a minesweeper game based on terminal written by rust.

I wrote it, just for practice.

## screenshot 
||
|-|
|![image](https://github.com/orzation/rua/assets/94043894/80edbce2-d498-4741-8fa6-a92032db2e8d)|



||
|-|
|![image](https://github.com/orzation/rua/assets/94043894/66e66537-da58-4b35-ab71-bc3f276368e4)|

||
|-|
|![image](https://github.com/orzation/rua/assets/94043894/2d88553a-0116-4bce-b78d-d0e553128f56)|

## install
### cargo

```sh
cargo install --git https://github.com/orzation/rua --branch master
```

## keymap

set keymap through env.

```sh
# example
UP_KEY=e DOWN_KEY=n LEFT_KEY=y RIGHT_KEY=o MINE_KEY=' ' FLAG_KEY=f QUIT_KEY=q rua
```

[default key is here](https://github.com/orzation/rua/blob/5d526754b596651e246c2dcf524f2ed092d6230f/src/config.rs#L27-L33)

you can also use mouse to control.
