text = "";
stage = 0;
placeholder = "enter ip";

ip = "";
port = "";



room_goto(rm_game);

with instance_create_layer(0,0,"Instances", obj_network) {
	socket = network_create_socket(network_socket_tcp);
	network_connect_raw(socket, "192.168.2.12", 8000); 
}