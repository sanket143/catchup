<script setup lang="ts">
const data = [
  {
    name: 'Codeforces',
    link: 'https://codeforces.com/problemset',
  },
  {
    name: 'CodeChef',
    link: 'https://www.codechef.com/practice/recent',
  },
  {
    name: 'AtCoder',
    link: 'https://atcoder.jp/contests/archive',
  },
  {
    name: 'Yukicoder',
    link: 'https://yukicoder.me/problems/',
  },
  {
    name: 'LeetCode',
    link: 'https://leetcode.com/problemset/',
  },
  {
    name: 'HackerRank',
    link: 'https://www.hackerrank.com/dashboard',
  },
  {
    name: 'CSES',
    link: 'https://cses.fi/problemset',
  },
  {
    name: 'CatchUp (GitHub)',
    link: 'https://github.com/sanket143/catchup',
  },
]
</script>

<template>
  <div class="container">
    <div class="title">Quick links</div>
    <ul class="links">
      <li v-for="quickLink in data">
        <a :href="quickLink.link" target="_blank">{{ quickLink.name }}</a>
      </li>
    </ul>
  </div>
</template>

<style scoped>
div.container {
  font-weight: bold;
  padding: 10px;
}

div.title {
  font-weight: bold;
  padding-bottom: 10px;
}

a {
  color: var(--color-text-link);
}

ul {
  padding-left: 20px;
}
</style>
