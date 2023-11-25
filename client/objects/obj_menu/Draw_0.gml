if (keyboard_check_pressed(vk_enter)) {
	
	if stage == 0 {
		ip = keyboard_string;
		keyboard_string = "";
		placeholder = "Enter the server port: ";
		stage++;
	} else {
		port = keyboard_string;
		
		room_goto(rm_game);

		with instance_create_layer(0,0,"Instances", obj_network) {
			socket = network_create_socket(network_socket_tcp);
			network_connect_raw(socket, other.ip, int64(other.port)); 
		}
		
		
		
	}

}

text = keyboard_string;
var _t = text;
if string_length(_t) == 0 {
	_t = placeholder;
}
draw_text(30, 40, _t);
