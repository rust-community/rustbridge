# Intro to Rust Slide Deck

## Instructions

The slideshow uses [Remark]. To change the content, edit the markdown in the
textarea in index.html. There are speaker notes; to use them, open index.html
in a browser and type 'c' to create a "cloned" view of the slideshow in another
window. Then in one of the windows, type 'p' to switch to presenter view. The
two windows will now change slides together.

[Remark]: https://remarkjs.com

Definitely tailor the content for your audience and your presenting style!
There's no "One Right Way" to teach Rust. Please send corrections, improvements,
and variations upstream!

There's a globe on the second slide for a fun way to keep track of all the
Rustbridge events that have taken place all over the world. To add your city,
edit globe.html and add your city's latitude and longitude to the `city_data`
javascript variable. Remember to send a pull request to this repo to record
your city for future workshops!

Note that the globe loads its data using ajax, so opening index.html as a local
file on your computer won't work. Serve the files from github pages or use
something like [live-server] or [SimpleHTTPServer] to serve the slides locally.

[live-server]: https://www.npmjs.com/package/live-server
[SimpleHTTPServer]: http://www.pythonforbeginners.com/modules-in-python/how-to-use-simplehttpserver/

## Credits

This was built starting from a slidedeck that [Ashley Williams created][agdubs].

[agdubs]: https://github.com/ashleygwilliams/a-very-brief-intro-to-rust
