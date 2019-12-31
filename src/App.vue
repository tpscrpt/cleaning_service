<template>
  <div id="app">
    <div class="items-masks-container" :style="{zIndex: 0}">
      <div class="item" v-for="(item, index) in items" :key="'item-' + index" :id="'item-' + index"
        :style="{
          backgroundColor: backgroundColor(index)
        }"
      >
        <span>{{item}}</span>
      </div>
    </div>

    <div id="svg-container" :style="{zIndex: 1}"></div>

    <div class="items-masks-container" :style="{zIndex: 2}" @mousemove="move" @touchmove="(ev) => move(ev, true)">
      <div class="mask" v-for="(item, index) in items" :key="'mask-' + index" :id="'mask-' + index">
      </div>
    </div>

    <div class="nags-container" ref="nags-container"></div>
  </div>
</template>

<script>
import { getCircleSize } from './utils'

export default {
  name: 'app',
  components: {
    
  },

  data: () => ({
    items: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    draw: null,
    movements: 0,
    width: window.innerWidth,
    height: window.innerHeight,
    grid: null,
    
    constants: {
      hash: '#',
      zeroPrefix: '000000000000000000',
      size: getCircleSize(window)
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

    // "Cleaning" part of the UI to see the website's content
    move(ev, mobile = false) {
      // extract mouse location variables
      const { clientX: x, clientY: y } = !mobile ? ev : ev.changedTouches[0]

      // Make SVG around mouse location transparent
      this.mask.add(
        this.draw.rect(this.constants.size)
                 .radius(this.constants.size / 10)
                 .center(x, y)
                 .fill({ color: '#000' })
      )

      let size = this.constants.size
      size = size % 2 == 0 ? size : size - 1

      // mark grid layer as masked around mouse area
      // we can now know which part of the site is
      // free of clutter for the user
      for (let i = 0; i < size; i ++) {
        for (let j = 0; j < size; j ++) {
          this.grid[x - size/2 + i][y - size / 2 + j] = 1
        }
      }

      // Increment and save value of number of mouse movements
      const movements = this.movements ++

      // every X mousemovements, nag the user by appending a div
      // near their mouse location (because they hate cleaning,
      // should hire us)
      if (movements % 84 === 0 && movements < 700) {
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
  
      this.constants.size = getCircleSize(window)

      const oldWidth = this.grid.length
      const oldHeight = this.grid[0].length
      this.width = window.innerWidth
      this.height = window.innerHeight
      
      // add new width column for width difference
      if (oldWidth < this.width) {
        const difference = this.width - oldWidth
        const col = new Array(this.height)
        col.fill(0)

        for (let i = 0; i < difference; i ++) {
          this.grid.push(col)
        }
      }

      // extend each old height column by the difference
      if (this.height > oldHeight) {
        const difference = this.height - oldHeight
        const extension = new Array(difference)
        extension.fill(0)

        for (let i = 0; i < oldWidth; i ++) {
          this.grid[i].push(...extension)
        }
      }
    }
  },

  created() {

  },

  mounted() {
    // populate the tracking grid based on screen dimensions
    const col = new Array(this.height)
    col.fill(0)
    this.grid = new Array(this.width)
    this.grid.fill(col)

    // mount the svg layer (interaction setup)
    this.draw = this.$svg('svg-container').size('100%', '100%')
    
    const image = this.draw.image('https://upload.wikimedia.org/wikipedia/commons/a/a9/Japanese_Wave_Pattern.svg').attr('preserveAspectRatio', 'xMaxYMax slice').size('100%', '100%')
    const rect = this.draw.rect('100%', '100%').fill({ color: '#fff' })
    const circle = this.draw.circle(100).fill({ color: '#000' })

    this.mask = this.draw.mask().add(rect).add(circle)

    image.maskWith(this.mask)

    // track window resize to expand the grid if needed
    window.addEventListener('resize', this.expand)
  }
}
</script>

<style>
body {
  margin: 0
}
#svg-container {
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
}
.items-masks-container {
  display: flex;
  flex-wrap: wrap;
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
  align-items: flex-start;
  justify-content: center;
  align-content: flex-start;
}
.item, .mask {
  width: 15vmax;
  height: 15vmax;
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
}
.nags-container > div {
  position: absolute;
  width: 50px;
  height: 50px;
  animation: disappear 3s forwards;
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
