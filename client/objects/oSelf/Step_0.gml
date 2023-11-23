var up = keyboard_check(ord("W"));
var left = keyboard_check(ord("A"));
var down = keyboard_check(ord("S"));
var right = keyboard_check(ord("D"));

if up or left or down or right {
	

	var hsp = right - left;
	var vsp = down - up;
	
	x += hsp * 5;
	y += vsp * 5;
	
	send({
		
		type: "pos",
		x: x,
		y: y,
		id: uuid
	});
	
}
