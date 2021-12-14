build:
	R -e "rextendr::document()"
	R -e "devtools::build()"