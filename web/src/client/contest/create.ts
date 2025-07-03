import graphqlRequest from '@/client/graphqlRequest'

const query = `
  mutation createContest($input: CreateContestInput!) {
    createContest(input: $input) {
      id
    }
  }
`

export default ({ input }) => {
  return graphqlRequest({
    query,
    variables: {
      input,
    },
  })
}
