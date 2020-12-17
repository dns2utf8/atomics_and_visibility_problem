pdf: slides.tex
	xelatex -interaction=nonstopmode --shell-escape slides.tex || true
	bibtex slides
	xelatex -interaction=nonstopmode --shell-escape slides.tex

clean:
	cd example && cargo clean
	git ls-files -o | xargs rm 
