When converting a float brightness value to greyscale, the tempting thing to do is to cast to int, and paste that three times to get an RGB color: `(value, value, value)`. However, there's a neat thing we can do - we don't have to increment every component at the same time:

* RGB=`(20, 20, 20)` - grey pixel
* RGB=`(20, 20, 21)` - a little brighter, slightly hue-shifted
* RGB=`(20, 21, 21)` - even brighter
* RGB=`(21, 21, 21)` - the next traditional greyscale step

So, when converting our float brightness to RGB, and our brightness falls in between two greyscale values, twiddle each component to match it more accurately! Specifically, for example, randomly choose between `(20, 20, 21)`, `(20, 21, 20)`, `(21, 20, 20)`, which I assume are all the same brightness. The human eye is much more accurate at picking out brightness inconsistencies rather than color inconsistencies, so it's a good trade-off - trading hue accuracy for brightness accuracy.

Inspired by https://photo.stackexchange.com/questions/113894/pseudo-high-bit-grayscale-does-this-idea-already-exist