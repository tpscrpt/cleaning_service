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
    <div class="items-masks-container" :style="{zIndex: 2}">
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
    backgroundColor (index) {
      return '#' + ('000000000000000000' + (1000000 * index).toString('16')).substr(-6)
    }
  },

  created () {

  },

  mounted () {
    this.draw = this.$svg('svg-container').size('100%', '100%')
    
    const image = this.draw.image('https://i.pinimg.com/originals/61/e7/8b/61e78b08a8dd18779132812218a9f2a8.jpg')
    const rect = this.draw.rect('100%', '100%').fill({ color: '#fff' })
    const circle = this.draw.circle(100).fill({ color: '#000' })

    this.mask = this.draw.mask().add(rect).add(circle)
    
    image.maskWith(this.mask)
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
