namespace math is

	namespace Calculus is
		const HARD: bool = true
	end

	const PI: f32 = 3.1415
	const E: f32 = 2.7182

	enum Sign is
		Positive
		Negative
	end

	struct Vector is
		magnitude: f32
		angle: i32
	end

	Vector::Vector(mag: f32, angle: f32) do
		this.magnitude = mag
		this.angle = angle
	end

	Vector::add(v: Vector) do
		this.magnitude += v.magnitude
		this.angle += v.angle
	end
end

struct Instance is
	is_running: bool
	speed: math::Vector
end

-- this . is_running = true
Instance::Instance() do
	this.is_running = true
	this.speed = math::Vector(0,0)
end

-- Main Entry Point for function
main(): i32 do
	a: math::Vector = math::Vector(1,2)
	b: math::Vector = math::Vector(1,2)
	c: math::Vector = a
	c.add(b)
	if c.angle >= 360 then
		a.add(b)
		-- print something about full circle
	elif c.angle < 0 then
		b.add(c)
		-- print something about going in reverse
	else
		c.add(a)
		-- print something about going no were
	end

	while c.angle < 360 do
		c.angle += 5
	end
	return 0
end
