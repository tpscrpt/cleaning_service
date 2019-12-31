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
    <div class="items-masks-container" :style="{zIndex: 2}" @mousemove="move" @touchmove="mobileMove">
      <div class="mask" v-for="(item, index) in items" :key="'mask-' + index" :id="'mask-' + index">
      </div>
    </div>
  </div>
</template>

<script>

export default {
  name: 'app',
  components: {
    
  },

  data: () => ({
    items: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    draw: null
  }),

  methods: {
    backgroundColor(index) {
      return '#' + ('000000000000000000' + (1000000 * index).toString('16')).substr(-6)
    },

    clear(x, y, size = 100, color = '#000') {
      this.mask.add(
        this.draw.circle(size).center(x, y).fill({ color })
      )
    },

    move(ev) {
      this.clear(ev.clientX, ev.clientY)
    },

    mobileMove(ev) {
      const { clientX, clientY } = ev.changedTouches[0]
      this.clear(clientX, clientY, 50)
    }
  },

  created() {

  },

  mounted() {
    this.draw = this.$svg('svg-container').size('100%', '100%')
    
    const image = this.draw.image('https://upload.wikimedia.org/wikipedia/commons/a/a9/Japanese_Wave_Pattern.svg').attr('preserveAspectRatio', 'xMaxYMax slice').size('100%', '100%')
    const rect = this.draw.rect('100%', '100%').fill({ color: '#fff' })
    const circle = this.draw.circle(100).fill({ color: '#000' })

    this.mask = this.draw.mask().add(rect).add(circle)
    
    image.maskWith(this.mask)
    setTimeout(() => this.clear(200, 200, '#777'), 3000)
    setTimeout(() => this.clear(220, 220, '#000'), 6000)
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
</style>
