import { createFileRoute } from '@tanstack/react-router'

const Account = () => {
  return (
    <div>
      <h1>Account</h1>
      <button>Change email</button>
      <button>Change password</button>
      <button>Download data</button>
      <button>Delete account</button>
    </div>
  )
}

export const Route = createFileRoute('/play/account')({ component: Account })
