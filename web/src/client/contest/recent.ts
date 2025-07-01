import axios from 'axios'
import graphqlRequest from '@/client/graphqlRequest'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()
const query = `
  query getUser($username: String!) {
    user(username: $username) {
      id
      username
      recentContest {
        id
      }
    }
  }
`

export default () => {
  return graphqlRequest({
    query,
    variables: {
      username: userStore.username,
    },
  })
}
