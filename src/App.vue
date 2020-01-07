<template>
  <div id="app">
    <div class="items-masks-container"
      :style="{zIndex: 0}"
    >
      <div class="item"
        v-for="(item, index) in items"
        :key="'item-' + index"
        :id="'item-' + index"
        :style="{
          width:  constants.sizing ? constants.sizing + 'px' : 0,
          height: constants.sizing ? constants.sizing + 'px' : 0,
        }"
        @click="() => open(index)"
      >
        <div
          :style="{
            width: '100%',
            height: '100%'
          }"
        >
          <img :src="item.img" class="item-svg"
            width="100%"
            preserveAspectRatio="xMidYMid meet"
          />
        </div>
      </div>
    </div>

    <div id="svg-container" :style="{zIndex: 1}"></div>

    <div class="items-masks-container"
      :style="{zIndex: 2}"
      @mousemove="move" @touchmove="(ev) => move(ev, true)"
    >
      <div class="mask"
        v-for="(item, index) in items"
        :key="'mask-' + index"
        :id="'mask-' + index"
        @click="(ev) => click(ev, index)"
        :style="{
          width:  constants.sizing ? constants.sizing + 'px' : 0,
          height: constants.sizing ? constants.sizing + 'px' : 0,
        }"
      >
      </div>
    </div>

    <div class="nags-container" ref="nags-container"></div>

    <div v-if="info != null" class="info-container">
      <span @click="close">{{items[info].message}}</span>
    </div>

  </div>
</template>

<script>
import * as math from 'mathjs'
import _items from './items'

export default {
  name: 'app',
  components: {
    
  },

  data: () => ({
    items: _items,
    draw: null,
    movements: 0,
    width: window.innerWidth,
    height: window.innerHeight,
    grid: null,
    info: null,
    
    constants: {
      hash: '#',
      zeroPrefix: '000000000000000000',
      size: null,
      halfSize: null,
      replacementMatrix: null,
      sizing: null
    }

  }),

  methods: {
    backgroundColor(index) {
      // fixed increment based on index, scaled by random
      return this.constants.hash + 
            (this.constants.zeroPrefix + 
              (1000000 * index * Math.random()).toString('16')
            ).substr(-6)
    },
    open(index) {
      this.info = index
    },
    close() {
      this.info = null
    },
    click(ev, index) {
      const { clientX: x, clientY: y } = ev

      if (this.grid._data[x][y]) 
        document.getElementById(`item-${index}`)
        .dispatchEvent(new Event('click'))
    },

    // "Cleaning" part of the UI to see the website's content
    move(ev, mobile = false) {
      // extract mouse location variables
      const { clientX: x, clientY: y } = !mobile ? ev : ev.changedTouches[0]

      // check if area was already cleaned
      // const wasCleaned = math.subset(this.grid, math.index(x, y)) === 1

      // Make SVG around mouse location transparent
      this.mask.add(
        this.draw.rect(this.constants.size, this.constants.size)
                 .radius(this.constants.size / 10)
                 .center(x, y)
                 .fill({ color: '#000' })
      )

      // mark grid layer as masked around mouse area
      // we can now know which part of the site is
      // free of clutter for the user
      //this.open([this.pos(x - this.constants.halfSize), this.pos(x + this.constants.halfSize), this.pos(y - this.constants.halfSize), this.pos(y + this.constants.halfSize)])
      this.grid.subset(
        math.index(
          math.range(this.pos(x - this.constants.halfSize), this.pos(x + this.constants.halfSize)),
          math.range(this.pos(y - this.constants.halfSize), this.pos(y + this.constants.halfSize))),
        this.cut(x, y)
      )


      // Increment and save value of number of mouse movements
      const movements = this.movements ++

      // every X mousemovements, nag the user by appending a div
      // near their mouse location (because they hate cleaning,
      // should hire us)
      if (movements % 80 === 0 && movements < 500) {
        const nagContainer = document.createElement('div')
        const nagMessage = document.createElement('span')
          nagMessage.innerText = 'TEST!'

        nagContainer.appendChild(nagMessage)
        nagContainer.style.left = x + 'px'
        nagContainer.style.top = y + 'px'
        nagContainer.style.transform = `translateX(${Math.random() * 10 + 'vw'}),`
                                     + `translateY(${Math.random() * 10 + 'vh'})`

        this.$refs['nags-container'].appendChild(nagContainer)
        
        // remove the nag to prevent memory leak
        setTimeout(() => this.$refs['nags-container'].removeChild(nagContainer), 5000)
      }
    },

    expand() {
      // prevent resizing too many times during 1 action
      if (this.lastExpanded > Date.now() - 1000) return
      this.lastExpanded = Date.now()
  
      this.setCircleSize()
      this.setHalfSize()
      this.setReplacementMatrix()
      this.setItemSize()

      const gridSize = math.size(this.grid)._data

      const oldWidth = gridSize[0]
      const oldHeight = gridSize[1]
      this.width = window.innerWidth
      this.height = window.innerHeight
      
      this.grid.resize([
        this.width > oldWidth ? this.width : oldWidth,
        this.height > oldHeight ? this.height : oldHeight
      ])
    },

    setCircleSize() {
      this.constants.size = 
        (window.innerWidth > window.innerHeight ? 
         window.innerWidth : window.innerHeight) / 10
    },
    setHalfSize() {
      let size = Math.floor(this.constants.size)
      size = size % 2 == 0 ? size : size - 1
      this.constants.halfSize = size / 2
    },
    setReplacementMatrix() {
      this.constants.replacementMatrix = math.ones(this.constants.halfSize * 2, this.constants.halfSize * 2)
    },
    setItemSize() {
      const items = this.items.length
      const width = this.width
      const height = this.height

      const x = width, y = height, n = items
      const px = Math.ceil(Math.sqrt(n * x / y))
      let sx , sy

      if(Math.floor(px * y / x) * px < n)  //does not fit, y/(x/px)=px*y/x
        sx = y / Math.ceil(px * y / x)
      else
        sx = x / px
      
      let py = Math.ceil(Math.sqrt(n * y / x))

      if(Math.floor(py * x / y) * py < n)  //does not fit
        sy = x / Math.ceil(x * py / y)
      else
        sy = y / py
      
      this.constants.sizing = sy > sx ? sy : sx
    },
    pos(val) {
      return val > 0 ? Math.floor(val) : 0
    },
    cut(x, y) {
      const matrix = this.constants.replacementMatrix
      const halfSize = this.constants.halfSize

      const cutX = x - halfSize <= 0
      const cutY = y - halfSize <= 0

      return !cutX && !cutY ?
        matrix :
        math.subset(
          matrix,
          math.index(
            cutX ? math.range(Math.ceil(Math.abs(x - halfSize)), halfSize * 2) : math.range(0, halfSize * 2),
            cutY ? math.range(Math.ceil(Math.abs(y - halfSize)), halfSize * 2) : math.range(0, halfSize * 2)
          )
        )
    }
  },

  created() {

  },

  mounted() {
    // populate the tracking grid based on screen dimensions
    this.grid = math.zeros(this.width, this.height)
    this.expand()

    // mount the svg layer (interaction setup)
    this.draw = this.$svg('svg-container').size('100%', '100%')
    
    const image = this.draw.image('https://upload.wikimedia.org/wikipedia/commons/a/a9/Japanese_Wave_Pattern.svg')
      .size('100%', '100%')
      
    if (window.chrome) image.attr('class', 'chrome-fix').attr('preserveAspectRatio', 'none')
    else image.attr('preserveAspectRatio', 'xMinYMax slice')

    const rect = this.draw.rect('100%', '100%').fill({ color: '#fff' })

    this.mask = this.draw.mask().add(rect)

    image.maskWith(this.mask)

    // track window resize to expand the grid if needed
    window.addEventListener('resize', this.expand)
  }
}
</script>

<style>
body {
  margin: 0;
  overflow: hidden;
}

#svg-container {
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  overflow: hidden
}
.items-masks-container {
  display: flex;
  flex-wrap: wrap;
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
  align-items:baseline;
  justify-content: center;
  align-content: space-around;
  overflow: hidden;
  user-select: none;
}
.item, .mask {
  display: flex;
  align-items: center;
  justify-content: center;
}
.item > div, .mask > div {
  margin: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.nags-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1;
  overflow: hidden;
}
.nags-container > div {
  position: absolute;
  width: 50px;
  height: 50px;
  animation: disappear 3s forwards;
}

.info-container {
  z-index: 3;
  display: flex;
  align-items: center;
  justify-content: center;
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

@media (orientation: landscape) {
  .chrome-fix {
    width: 100vw;
    height: 100vw;
  }
}
@media (orientation: portrait) {
  .chrome-fix {
    width: 100vh;
    height: 100vh;
  }
}
@keyframes disappear {
  0% {
    opacity: 1
  }
  100% {
    opacity: 0
  }
}
</style>
