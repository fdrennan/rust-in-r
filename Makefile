build:
	R -e "devtools::document()"
	R -e "rextendr::document()"
	R -e "devtools::build()"

