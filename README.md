# Meme Cutter
Remake of my old MemeCutter (written in C#) in Rust.

This app is for cutting blank areas around images. It is named like this
because its original purpose was to cut blank black areas out of meme
screenshots.

Compared to the old version, this is approx. 13× faster and uses 6× less memory

## Example
Lets say you have an image `image.png` (if you have dark theme open the image in new tab to see it more clearly):

![image](https://user-images.githubusercontent.com/46282097/219484635-1e059ea8-123d-4cc0-bb1a-73345ae177a9.png)

As you can see the github logo has edges with blank area around. You can cut these edges with meme-cutter like this:
```shell
 > meme-cutter file image.png result.png
```
Now you have a new image `result.png` without the blank areas around (if you have dark theme open the image in new tab to see it more clearly):

![result](https://user-images.githubusercontent.com/46282097/219485188-a1bf1f52-4d90-4028-9b2e-af36358e51f0.png)

meme-cutter can also cut whole folders with images.

If you want to know more use `meme-cutter help` to show the help.

## Links
- **Author:** [BonnyAD9](https://github.com/BonnyAD9)
- **GitHub repository:** [BonnyAD9/meme-cutter](https://github.com/BonnyAD9/meme-cutter)
- **My Website:** [bonnyad9.github.io](https://bonnyad9.github.io/)
