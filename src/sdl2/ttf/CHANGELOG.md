# Changelog
## 0.14.0
### Changes
- Added a changelog
- The ttf context is now needed to create fonts, which should make it more apparent that it needs to live
- Changed some i32 indices to u16 (mainly widths, where subzero should be represented with an ```Option``` anyway)
- Moved font creation to the TTF context
- Changed the render method to use a builder pattern
- Updated the demo example to work with the above changes
- The order of arguments when loading indexed fonts was changed, to first select the font/index and then its size
- The functions of the 'load from RWops' extension trait was moved to the TTF context too
- ```Font.size``` has been split into ```Font.size_of``` (for UTF-8), ```Font.size_of_latin1``` (for Latin-1) and ```Font.size_of_char```
- ```Font.render``` has been split into ```Font.render``` (for UTF-8), ```Font.render_latin1``` (for Latin-1) and ```Font.render_char```
- The font size checks should now properly use UTF-8. (It used to rely on the Latin-1 function)

### Renames
- ```Font.get_outline``` -> ```Font.get_outline_width```
- ```Font.set_outline``` -> ```Font.set_outline_width```
- ```Font.line_skip``` -> ```Font.recommended_line_spacing```
- ```Font.faces``` -> ```Font.face_count```
- ```Font.index_of_char``` -> ```Font.find_glyph```
- ```Font.metrics_of_char``` -> ```Font.find_glyph_metrics```
