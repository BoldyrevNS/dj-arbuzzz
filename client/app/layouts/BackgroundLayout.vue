<script setup lang="ts">
type Circle = {
	x: number;
	y: number;
	vx: number;
	vy: number;
	radius: number;
	vr: number;
	minR: number;
	maxR: number;
	hue: number;
	sat: number;
	light: number;
	a: number;
};

const canvasRef = ref<HTMLCanvasElement | null>(null);
const containerRef = ref<HTMLDivElement | null>(null);

let ctx: CanvasRenderingContext2D | null = null;
let animationId = 0;
let circles: Circle[] = [];
let resizeObserver: ResizeObserver | null = null;
let lastTime = 0;

function rand(min = 0, max = 1) {
	return Math.random() * (max - min) + min;
}

function randInt(min = 0, max = 1) {
	return Math.floor(rand(min, max + 1));
}

function randomColor() {
	const hue = randInt(0, 360);
	const sat = randInt(20, 50);
	const light = randInt(30, 60);
	const a = rand(0.25, 0.7);
	return { hue, sat, light, a };
}

function createCircle(
	left: number,
	top: number,
	width: number,
	height: number,
): Circle {
	const { hue, sat, light, a } = randomColor();
	const minR = rand(40, 80);
	const maxR = rand(80, 160);
	return {
		x: rand(left, left + width),
		y: rand(top, top + height),
		vx: rand(-30, 30) / 60,
		vy: rand(-30, 30) / 60,
		radius: rand(minR, maxR),
		vr: rand(-0.01, 0.01),
		minR,
		maxR,
		hue,
		sat,
		light,
		a,
	};
}

function resizeCanvasToParent() {
	const canvas = canvasRef.value;
	const container = containerRef.value;
	if (!canvas || !container) return;
	const rect = container.getBoundingClientRect();
	const width = Math.max(1, Math.floor(rect.width));
	const height = Math.max(1, Math.floor(rect.height));
	const dpr = Math.max(1, window.devicePixelRatio || 1);

	canvas.width = Math.floor(width * dpr);
	canvas.height = Math.floor(height * dpr);

	canvas.style.width = `${width}px`;
	canvas.style.height = `${height}px`;

	ctx = canvas.getContext('2d');
	if (!ctx) return;
	ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

	const area = width * height;

	const referenceArea = 375 * 667;
	const baseCountForReference = 12;
	const scale = Math.sqrt(area / referenceArea);
	let targetCount = Math.round(baseCountForReference * scale);

	targetCount = Math.max(8, Math.min(200, targetCount));

	if (circles.length === 0) {
		const left = rect.left;
		const top = rect.top;
		circles = Array.from({ length: targetCount }, () =>
			createCircle(left, top, width, height),
		);
	}
}

function draw(timestamp: number) {
	if (!ctx) return;
	if (!lastTime) lastTime = timestamp;
	const dt = Math.min(0.2, timestamp - lastTime);
	lastTime = timestamp;

	const canvas = canvasRef.value;
	const container = containerRef.value;
	if (!canvas || !container) return;
	const rect = container.getBoundingClientRect();
	const width = rect.width;
	const height = rect.height;

	const margin = Math.max(width, height) * 0.6 + 150;
	const boundLeft = rect.left - margin;
	const boundTop = rect.top - margin;
	const boundRight = rect.left + width + margin;
	const boundBottom = rect.top + height + margin;

	ctx.clearRect(0, 0, width, height);

	for (const c of circles) {
		c.x += c.vx * dt;
		c.y += c.vy * dt;

		c.radius += c.vr * dt;
		if (c.radius < c.minR) {
			c.radius = c.minR;
			c.vr = Math.abs(c.vr);
		}
		else if (c.radius > c.maxR) {
			c.radius = c.maxR;
			c.vr = -Math.abs(c.vr);
		}

		if (Math.random() < 0.001 * (dt / 16)) {
			c.vx += rand(-0.2, 0.2);
			c.vy += rand(-0.2, 0.2);
			c.vr += rand(-0.003, 0.003);
		}

		if (Math.random() < 0.001 * (dt / 16)) {
			c.hue = (c.hue + rand(-10, 10) + 360) % 360;
			c.sat = Math.max(15, Math.min(60, c.sat + rand(-6, 6)));
			c.light = Math.max(25, Math.min(65, c.light + rand(-6, 6)));
		}

		if (c.x < boundLeft) {
			c.x = boundLeft;
			c.vx = Math.abs(c.vx) * 0.6 + 0.01;
			c.vy += rand(-0.05, 0.05);
		}
		else if (c.x > boundRight) {
			c.x = boundRight;
			c.vx = -Math.abs(c.vx) * 0.6 - 0.01;
			c.vy += rand(-0.05, 0.05);
		}

		if (c.y < boundTop) {
			c.y = boundTop;
			c.vy = Math.abs(c.vy) * 0.6 + 0.01;
			c.vx += rand(-0.05, 0.05);
		}
		else if (c.y > boundBottom) {
			c.y = boundBottom;
			c.vy = -Math.abs(c.vy) * 0.6 - 0.01;
			c.vx += rand(-0.05, 0.05);
		}

		const drawX = c.x - rect.left;
		const drawY = c.y - rect.top;

		ctx.beginPath();
		ctx.fillStyle = `hsla(${c.hue.toFixed(0)}, ${c.sat.toFixed(0)}%, ${c.light.toFixed(
			0,
		)}%, ${c.a.toFixed(2)})`;
		ctx.arc(drawX, drawY, c.radius, 0, Math.PI * 2);
		ctx.fill();
	}

	animationId = requestAnimationFrame(draw);
}

onMounted(() => {
	resizeCanvasToParent();

	animationId = requestAnimationFrame(draw);
});

onBeforeUnmount(() => {
	if (animationId) cancelAnimationFrame(animationId);
	if (resizeObserver && containerRef.value) {
		resizeObserver.unobserve(containerRef.value);
		resizeObserver.disconnect();
		resizeObserver = null;
	}
	else {
		window.removeEventListener('resize', resizeCanvasToParent);
	}
});
</script>

<template>
	<div
		ref="containerRef"
		class="container"
	>
		<div class="backdrop" />
		<canvas ref="canvasRef" />
		<div class="content">
			<slot />
		</div>
	</div>
</template>

<style scoped lang="scss">
.container {
  height: 100%;
  width: 100%;
  position: absolute;
  overflow: hidden;
}

.content {
  position: relative;
  height: 100%;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 2;
}

.backdrop {
  z-index: 1;
  position: fixed;
  left: 0;
  top: 0;
  height: 100%;
  width: 100%;

  box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
}

canvas {
  position: fixed;
  top: 0;
  left: 0;
  overflow: hidden;
}
</style>
