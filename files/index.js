const express=require("express")
const app=express();

app.get("/space.hdr",(req,res,next)=>{
	res.send("test");
})

app.listen(8000,()=>{
	console.log("running");
})
