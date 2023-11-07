# bevy_pixel_pefect

![A showcase of bevy_pixel_perfect](/showcase.gif)

A pixel perfect post processing effect based on [Aarthificial's Astortion renderer](https://www.youtube.com/watch?v=jguyR4yJb1M).

## Usage

1. Use nearest neighbor filtering. (`ImagePlugin::default_nearest()`)
1. Add the `PixelPerfectPlugin` plugin to your app.
1. Add the `PixelPerfectCamera` component to your camera and set the `resolution` field to the desired virtual resolution.
1. Move the camera with `subpixel_position` instead of translation.

Check `examples` for full usage example.

## Features

### `bilinear` (default)

The `bilinear` feature enables bilinear sampling in the shader. This reduces aliasing in the final texture at the cost of minorly increasing the
cost of the upscale. **Do not disable nearest neighbor filtering when using this feature, the bilinear sampling is done in shader due to how the upscaling works**.
If you're experiencing artifacts with this feature enabled try disabling it.

|`bevy`|`bevy_pixel_pefect`|
|------|-------------------|
|`0.12`|`0.1`              |