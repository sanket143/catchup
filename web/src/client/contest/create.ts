import graphqlRequest from '@/client/graphqlRequest'

const query = `
  mutation createContest($input: CreateContestInput!) {
    createContest(input: $input) {
      id
    }
  }
`

type CreateContestInput = { input: { name: string } }

export default ({ input }: CreateContestInput) => {
  return graphqlRequest({
    query,
    variables: {
      input,
    },
  })
}
