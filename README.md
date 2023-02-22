<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 -->
<!-- inline html -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# 🌈 Tony McMapface 🌈

Takes HDR Rec.709/sRGB stimulus, and maps it to LDR. It's tranquil and collected, and won't set your eyes ablaze.

<img src="https://user-images.githubusercontent.com/16522064/219526571-8e813b0a-c253-4706-b104-27d501ae24ce.jpg" width="600" height="auto" />

</div>

## About

Tony is a display transform intended for real-time applications such as games. It is intentionally _boring_, does not increase contrast or saturation, and stays close to the input stimulus where compression isn't necessary.

It ships as a LUT along with a bit of shader code for sampling. There's no tweakables; if you need a different look, do it before and/or after the display transform.

Please note that the shader output is linear, so if "gamma space" sRGB is desired, the sRGB OETF must be used afterwards.

There's also a basic OCIO config with sRGB output.

_Disclaimer: Tony McMapface is phenomenological, and tuned to its author's visual system (with mixed success). If you need something mathematically pure, try [AgX](https://github.com/sobotka/AgX)._

## Ports 💖

* [DaVinci Resolve port](https://github.com/SergeyMakeev/tony-mc-mapface-fuse) by Sergey Makeev

## Derivation

The generator's source code will come at a later time. Meanwhile, here's a TL;DR:

Tony McMapface takes a maximalist approach to the image formation process, and explicitly models perceptual phenomena:

* Brightness-equivalent luminance of the input stimulus is compressed (accounting for the [Helmholtz–Kohlrausch effect](https://en.wikipedia.org/wiki/Helmholtz%E2%80%93Kohlrausch_effect)).
  * The non-linearity resembles Reinhard.
* Color hues are preserved during compression, except for a deliberate [Bezold–Brücke shift](https://en.wikipedia.org/wiki/Bezold%E2%80%93Br%C3%BCcke_shift).
* To avoid posterization, selective desaturation is employed, with care to avoid the [Abney effect](https://en.wikipedia.org/wiki/Abney_effect).

As can be expected, the models underlying this process are imperfect, thus a liberal amount of eyeballing and fine tuning was applied.

## Examples

<div align="center">

<img src="https://user-images.githubusercontent.com/16522064/219526341-2d9c4932-496e-493f-8d8b-c49ab87ebbe1.jpg" width="600" height="auto" />
<img src="https://user-images.githubusercontent.com/16522064/219527156-cfa9dba0-f4fe-409c-8e60-1fe3f79dbb40.jpg" width="600" height="auto" />
<img src="https://user-images.githubusercontent.com/16522064/219536385-5e80e563-779b-427e-a315-6a682d709f56.jpg" width="600" height="auto" />
<img src="https://user-images.githubusercontent.com/16522064/219536271-07fd1e01-3d61-4922-8611-823cd056cb35.jpg" width="600" height="auto" title="EV+1" />
<img src="https://user-images.githubusercontent.com/16522064/219536784-a0435a94-1d2d-4b37-8629-e6abc8d02894.jpg" width="600" height="auto" />

HDR images from <https://github.com/sobotka/Testing_Imagery>

</div>

## Acknowledgments

Thanks to Chris Brejon, David Briggs, Jed Smith, Troy Sobotka, and Alex Tardif for their excellent write-ups about color.

Special thanks to Troy Sobotka for eye-opening discussions, and triggering my obsessions 🙈

## License

This contribution is dual licensed under EITHER OF

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
