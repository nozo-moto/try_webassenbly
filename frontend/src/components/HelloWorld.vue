<template>
  <div class="hello container form-group" style="">
    <p>{{ msg }}</p>
    <div ><button type="submit" @click="submitRust" style="text-align: center; border: 1px; margin: 5px" class="btn btn-default">submitRust</button></div>
    <div ><button type="submit" @click="submitJS" style="text-align: center; border: 1px; margin: 5px" class="btn btn-default">submitJS</button></div>
    <div ><button type="submit" @click="clear" style="text-align: center; border: 1px; margin: 5px" class="btn btn-default">clear</button></div>
</div>
</template>

<script>
export default {
  name: 'hello',
  data () {
    return {
      a: '',
      b: '',
      msg: ''
    }
  },
  methods: {
    submitRust: function () {
      const self = this
      self.msg = ''
      const startTime = new Date()
      fetch('static/wasm.wasm')
        .then((response) => response.arrayBuffer())
        .then((bytes) => WebAssembly.instantiate(bytes, {}))
        .then((results) => {
          const startTime2 = new Date()
          const instance = results.instance
          self.msg = instance.exports.sum(parseInt(self.a, 10), self.b)
          const endTime = new Date()
          self.msg = self.msg + ' :\t' + (endTime.getTime() - startTime.getTime() + 'ms') + ' :\t' + (endTime.getTime() - startTime2.getTime() + 'ms')
        })
    },
    submitJS: function () {
      const self = this
      self.msg = ''
      const startTime = new Date()
      self.matrix()
      const endTime = new Date()
      self.msg = (endTime.getTime() - startTime.getTime() + 'ms')
    },
    matrix: function () {
      let mat1 = new Array(1000)
      let mat2 = new Array(1000)
      let rlt = new Array(1000)
      for (let i = 0; i <= 1000; i++) {
        mat1[i] = new Array(1000)
        mat2[i] = new Array(1000)
        rlt[i] = new Array(1000)
        for (let j = 0; j <= 1000; j++) {
          mat1[i][j] = Math.random() * 11
          mat2[i][j] = Math.random() * 11
          rlt[i][j] = 0
        }
      }
      for (var i = 0; i < 8; i++) {
        for (var j = 0; j < 8; j++) {
          for (var k = 0; k < 8; k++) {
            rlt[i][j] += mat1[i][k] * mat2[k][j]
          }
        }
      }
    },
    clear: function () {
      const self = this
      self.msg = ''
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
  h1,
  h2 {
    font-weight: normal;
  }
  ul {
    list-style-type: none;
    padding: 0;
  }
  li {
    display: inline-block;
    margin: 0 10px;
  }
  a {
    color: #42b983;
  }
</style>
