import { Fragment } from 'react'
import { Disclosure, Menu, Transition } from '@headlessui/react'
import { useAuth0 } from '@auth0/auth0-react'

function classNames(...classes) {
	return classes.filter(Boolean).join(' ')
}

export default function Profile() {
	const { user, isAuthenticated, isLoading } = useAuth0();

	if (isLoading)
		return <div>Loading ...</div>;

	return (
		<div className='m-4 py-4'>
			<div className='flex items-center'>
				<Menu as="div" className="relative">
					<div className='flex items-center'>
						{/* Profile dropdown */}
						<Menu.Button className="bg-gray-800 mx-2 flex text-sm rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white">
							<span className="sr-only">Open user menu</span>
							<img
								src={user.picture}
								className="h-8 w-8 rounded-full"
								alt=""
							/>
						</Menu.Button>
						<p className='text-gray-300 mx-2 text-md'>{user.email}</p>
					</div>
					<Transition
						as={Fragment}
						enter="transition ease-out duration-100"
						enterFrom="transform opacity-0 scale-95"
						enterTo="transform opacity-100 scale-100"
						leave="transition ease-in duration-75"
						leaveFrom="transform opacity-100 scale-100"
						leaveTo="transform opacity-0 scale-95"
					>
						<Menu.Items className="origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg py-1 bg-white ring-1 ring-black ring-opacity-5 focus:outline-none">
							<Menu.Item>
								{({ active }) => (
									<a
										href="#"
										className={classNames(active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700')}
									>
										Your Profile
									</a>
								)}
							</Menu.Item>
							<Menu.Item>
								{({ active }) => (
									<a
										href="#"
										className={classNames(active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700')}
									>
										Settings
									</a>
								)}
							</Menu.Item>
							<Menu.Item>
								{({ active }) => (
									<a
										href="#"
										className={classNames(active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700')}
									>
										Sign out
									</a>
								)}
							</Menu.Item>
						</Menu.Items>
					</Transition>
				</Menu>
			</div>
		</div>
	)
}