
cwd=$(pwd)

dev:
	docker run -p8080:8080 -v $(pwd):/code -ti test 

build: 
	docker build -t test .