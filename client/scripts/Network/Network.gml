function process(_data) {

	show_debug_message(_data);
	
	switch _data[0] {
		
		case "Pos":
			show_debug_message("position send");
		break;
		
	}
	
}

function send(_data) {
	
	var _encoded = buffer_create(1, buffer_grow, 1);
	SnapBufferWriteMessagePack(_encoded, _data);
	
	var _size_encoded = buffer_tell(_encoded);
	var _buff_with_size = buffer_create(_size_encoded + 1, buffer_fixed, 1);
	buffer_write(_buff_with_size, buffer_u64, _size_encoded);
	buffer_copy(_encoded, 0, _size_encoded, _buff_with_size, 1);
	show_debug_message("Sent: ");
	show_debug_message(buffer_prettyprint(_buff_with_size));
	network_send_raw(socket, _buff_with_size, _size_encoded + 1);
	
}