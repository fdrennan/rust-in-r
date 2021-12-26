build:
	R -e "devtools::document()"
	R -e "rextendr::document()"
	R -e "devtools::build()"

style:
	R -e "styler::style_dir(path = '.', exclude_dirs = c('packrat', 'renv'))"