import { createFileRoute, Link } from '@tanstack/react-router'

export const Route = createFileRoute('/play/relay')({
  component: GalacticRelay,
})

function GalacticRelay() {
  return (
    <div>
      <h1>Galactic relay</h1>
      <div>Welcome to the galactic internet</div>
      <div>
        <Link to=".">Maps</Link>
        <Link to=".">Messages</Link>
        <Link to=".">Market place</Link>
      </div>
    </div>
  )
}
