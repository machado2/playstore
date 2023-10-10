import './App.css'
import { graphql } from 'relay-runtime'
import { useLazyLoadQuery } from 'react-relay'
import { AppGamesQuery } from './__generated__/AppGamesQuery.graphql'

const gamesQuery = graphql`
  query AppGamesQuery {
    listedApp {
      nodes {
      id,
      title
      }
    }
  }`;

const App = () => {
  const games = useLazyLoadQuery<AppGamesQuery>(gamesQuery, {})
    .listedApp.nodes;
  return (
    <ul>
      {games.map(game => (
        <li key={game.id}>{game.title}</li>
      ))}
    </ul>
  )
};

export default App
