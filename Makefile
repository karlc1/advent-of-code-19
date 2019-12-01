create: 
	@cargo new day$(day) --bin --vcs none &&\
	mkdir ./day$(day)/input && \
	touch ./day$(day)/input/input.txt && \
	rm ./day$(day)/src/main.rs && \
	cp ./template ./day$(day)/src/main.rs && \
	printf "\nCreated template for day $(day) - remember to copy the input to day$(day)/input/input.txt\n "

run: 
	@cd ./day$(day) && \
	echo " " && \
	cargo run $(day) $(part)
