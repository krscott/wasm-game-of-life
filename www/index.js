import { Universe, Cell } from "wasm-game-of-life"
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg"

const CELL_SIZE_PX = 5
const GRID_COLOR = "#CCCCCC"
const DEAD_COLOR = "#FFFFFF"
const ALIVE_COLOR = "#000000"

const universe = Universe.random(64, 64)
// const universe = Universe.from_str(`
// ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
// ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
// ◼◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
// ◻◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
// ◼◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
// ◻◼◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
// ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
// ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
// ◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
// `)

const glider = Universe.from_str(`
◻◻◼
◼◻◼
◻◼◼
`)

const spaceship = Universe.from_str(`
◼◻◻◼◻
◻◻◻◻◼
◼◻◻◻◼
◻◼◼◼◼
◻◻◻◻◻
`)

const pulsar = Universe.from_str(`
◻◻◼◼◼◻◻◻◼◼◼◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻
◼◻◻◻◻◼◻◼◻◻◻◻◼
◼◻◻◻◻◼◻◼◻◻◻◻◼
◼◻◻◻◻◼◻◼◻◻◻◻◼
◻◻◼◼◼◻◻◻◼◼◼◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◼◼◼◻◻◻◼◼◼◻◻
◼◻◻◻◻◼◻◼◻◻◻◻◼
◼◻◻◻◻◼◻◼◻◻◻◻◼
◼◻◻◻◻◼◻◼◻◻◻◻◼
◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◼◼◼◻◻◻◼◼◼◻◻
`)



const width = universe.width()
const height = universe.height()
let animationId = null
let ticksPerFrame = 1.0
let tickCounter = 0.0

const canvas = document.getElementById("game-of-life-canvas")
canvas.height = (CELL_SIZE_PX + 1) * height + 1
canvas.width = (CELL_SIZE_PX + 1) * width + 1

const ctx = canvas.getContext("2d")

const playPauseButton = document.getElementById("play-pause")
const randomizeButton = document.getElementById("randomize-button")
const clearButton = document.getElementById("clear-button")
const speedInput = document.getElementById("speed-input")


canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect()

    const scaleX = canvas.width / boundingRect.width
    const scaleY = canvas.height / boundingRect.height

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX
    const canvasTop = (event.clientY - boundingRect.top) * scaleY

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE_PX + 1)), height - 1)
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE_PX + 1)), width - 1)

    if (event.shiftKey) {
        stamp(row, col, spaceship)
    } else if (event.ctrlKey) {
        stamp(row, col, pulsar)
    } else if (event.altKey) {
        stamp(row, col, glider)
    } else {
        universe.toggle_cell(row, col)
    }

    draw()
})

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play()
    } else {
        pause()
    }
})

clearButton.addEventListener("click", event => {
    universe.clear()
    draw()
})

randomizeButton.addEventListener("click", event => {
    universe.randomize()
    draw()
})

speedInput.addEventListener("change", event => {
    ticksPerFrame = Math.pow(2, event.target.value) - 1

    console.log(ticksPerFrame)
})


draw()
play()


function renderLoop() {
    tickCounter += Math.max(0.01, ticksPerFrame)

    while (tickCounter >= 1.0) {
        tickCounter -= 1.0
        universe.tick()
    }

    draw()

    animationId = requestAnimationFrame(renderLoop)
}

function draw() {
    drawGrid()
    drawCells()
}

function stamp(row, col, pattern) {
    row = Math.floor(row - pattern.height() / 2)
    col = Math.floor(col - pattern.width() / 2)

    universe.insert_universe(row, col, pattern)
}

function isPaused() {
    return animationId === null
}

function play() {
    playPauseButton.textContent = "⏸"
    renderLoop()
}

function pause() {
    playPauseButton.textContent = "▶"
    cancelAnimationFrame(animationId)
    animationId = null
}

function drawGrid() {
    ctx.beginPath()
    ctx.strokeStyle = GRID_COLOR

    // Vertical lines
    for (let i = 0; i <= width; ++i) {
        ctx.moveTo(i * (CELL_SIZE_PX + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE_PX + 1) + 1, (CELL_SIZE_PX + 1) * height + 1)
    }

    // Horizontal lines
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE_PX + 1) + 1)
        ctx.lineTo((CELL_SIZE_PX + 1) * width + 1, j * (CELL_SIZE_PX + 1) + 1)
    }

    ctx.stroke()
}

function getIndex(row, column) {
    return row * width + column
}

function isBitSet(n, arr) {
    const byte = Math.floor(n / 8)
    const mask = 1 << (n % 8)
    return (arr[byte] & mask) === mask
}

function drawCells() {
    const cellsPtr = universe.cells()
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8)

    for (let row = 0; row < height; ++row) {
        for (let col = 0; col < width; ++col) {
            const idx = getIndex(row, col)

            ctx.fillStyle = (isBitSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR)

            ctx.fillRect(
                col * (CELL_SIZE_PX + 1) + 1,
                row * (CELL_SIZE_PX + 1) + 1,
                CELL_SIZE_PX,
                CELL_SIZE_PX
            )
        }
    }
}