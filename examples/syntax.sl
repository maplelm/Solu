namespace testing is
	struct TestObject is
		msg: String
		code: i32
	end

	is_testing(): bool do
		return true
	end
end

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
	name: String
end

-- Constructor --
Entity::Entity(x: i32, y:i32, c: char, n: String): Entity do
	n = entity with
		pos_x = x
		pos_y = y
		width = 1
		height = 1
		sprite = c
		name = n
end end

-- Method --
Entity::move(xd: i32, yd: i32) do this.pos_x += xd; this.pos_y += yd end
Entity::right() do this.pos_x += 1 end
Entity::left()  do this.pos_x -= 1 end
Entity::up()    do this.pos_y -= 1 end
Entity::down()  do this.pos_y += 1 end


-- Main Entry Point for function
main(): i32 do
	x: i32 = 1
	a: i32 = 10
	if true then
		x = 1
	end
	player: Entity = Entity with
		pos_x = 0b10000
		pos_y = 0xFF
		width = 1
		height = 1
		sprite = '\u0061'
		name = "player"
	end
	enemy: Entity = Entity(5, 5, 'E', "blob")
	-- some sort of standard library to print the state of the player and the enemy
	return a
end
