 
    
  

struct Map is
	width: i32
	length: i32
	depth: i32
	temp: f32
	tile_types : i32[10*10*10] 
end
struct Entity is
	pos_x: i32
	pos_y: i32
	width: i32
	height: i32
	sprite: char
	map: *Map
	ref: &Map
	name: String
end

Entity(x: i32, y:i32, c: char, n: String): Entity do
	n = entity with
		pos_x = x
		pos_y = y
		width = 1
		height = 1
		sprite = c
		map = nil
		ref = nil
		name = n
	end
end

Entity::move(xd: i32, yd: i32) do
	this.pos_x += xd
	this.pos_y += yd
end

-- Main Entry Point for function
main(): i32 do
	player: Entity = Entity with
		pos_x = 0b10000
		pos_y = 0xFF
		width = 1
		height = 1
		sprite = '\u0061'
		map = nil
		ref = nil
		name = "player"
	end
	enemy: Entity = Entity(5, 5, 'E', "blob")
	-- some sort of standard library to print the state of the player and the enemy
	return a
end
