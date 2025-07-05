import graphqlRequest from '@/client/graphqlRequest'

const query = `
  mutation evaluateContest($input: EvaluateContestInput!) {
    evaluateContest(input: $input) {
      id
      isEvaluated
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
