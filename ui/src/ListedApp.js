import graphql from 'babel-plugin-relay/macro';


import React from 'react';
import { useFragment } from 'react-relay';

export const ListedAppFragment = graphql`
  fragment ListedAppFragment on ListedApp {
    id,
    title,
    details,
  }
`;

export const ListedApp = ({ listedApp }) => {
  const game = useFragment(ListedAppFragment, listedApp);
  return <article>
    <a href={`https://play.google.com/store/apps/details?id=${game.id}`}>
      <h2>{game.title}</h2>
    </a>
    <div class="screenshots">
      
      {game.details.screenshots.slice(0, 3).map((screenshot, i) => (
        <img class="screenshot" src={screenshot} alt={game.title} />
      ))}
      
    </div>
    <div class="description" dangerouslySetInnerHTML={{__html: game.details.descriptionHTML}} title={game.details.description} />

  </article>
}
