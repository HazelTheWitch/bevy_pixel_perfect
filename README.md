# bevy_pixel_pefect

A pixel perfect post processing effect based on [Aarthificial's Astortion renderer](https://www.youtube.com/watch?v=jguyR4yJb1M).

## Usage

1. Use nearest neighbor filtering. (`ImagePlugin::default_nearest()`)
1. Add the `PixelPerfectPlugin` plugin to your app.
1. Add the `PixelPerfectCamera` component to your camera and set the `resolution` field to the desired virtual resolution.
1. Move the camera with `subpixel_position` instead of translation.

Check `examples` for full usage example.

|`bevy`|`bevy_pixel_pefect`|
|------|-------------------|
|`0.12`|`0.1`              |