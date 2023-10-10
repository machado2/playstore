import './App.css';
import { ListedApp } from './ListedApp';

import graphql from 'babel-plugin-relay/macro';
import { useLazyLoadQuery } from 'react-relay'


const gamesQuery = graphql`
  query AppGamesQuery {
    listedApp(
      pagination:{
        pages:{
          page: 1,
          limit: 20
        }
      }
      filters:{
        score: {
          gte:4,
        },
        minInstalls:{
          gte:10000
        },
        free: {
          eq:true
        }
      },
    ) {
      nodes {
        id
        ...ListedAppFragment
      }
    }
  }`;

function App() {
  const games = useLazyLoadQuery(gamesQuery, {})
    .listedApp.nodes;
  return (
    <>
        {games.map((game, i) => (
          <>
          <ListedApp key={game.id} listedApp={game} />
          </>
        ))}
    </>
  )
}

export default App;
