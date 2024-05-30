# cspmakeagame
this was supposed to be my make a game and i dont feel like changing all the make a game references

this is made using rust compiled to webassembly and distributed using docker containers and nginx reverse proxy running in an aws ec2 instance along with assets distributed using a node.js container (im def not a nerd)

this is a mix of code i wrote myself and stolen code

# build instructions

you need to install the rust toolchain, the rust wasm toolchain, node.js and npm, docker, and docker compose

if you have all the dependancies, just run the buildandrun.sh script and it should be accessable on http://localhost/

put an HDRI image in the .hdr format called "space.hdr" in the cspmakeagame/files folder
