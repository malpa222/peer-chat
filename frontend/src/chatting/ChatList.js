import { useState } from 'react'
import { RadioGroup } from '@headlessui/react'

const chats = [
	{
		name: 'Lorem',
		msg: 'Ipsum',
	},
	{
		name: 'Dolor',
		msg: 'Sit',
	},
	{
		name: 'Andrzej',
		msg: 'Matus',
	},
	{
		name: 'Albrecht',
		msg: 'Hohenzollern',
	},
]

export default function Example() {
	const [selected, setSelected] = useState(chats[0])

	return (
		<div className="w-full py-8">
			<div className="w-11/12">
				<RadioGroup value={selected} onChange={setSelected}>
					<div className="space-y-2">
						{chats.map((chat) => (
							<RadioGroup.Option
								key={chat.name}
								value={chat}
								className={({ active, checked }) =>
									`${active
										? 'ring-2 ring-white ring-opacity-60 ring-offset-2 ring-offset-sky-300'
										: ''
									}
                  ${checked ? 'bg-sky-900 bg-opacity-75 text-white' : 'bg-white'
									}
                    relative flex cursor-pointer rounded-lg px-5 py-4 shadow-md focus:outline-none`
								}
							>
								{({ active, checked }) => (
									<>
										<div className="flex w-full items-center justify-between">
											<div className="flex items-center">
												<div className="text-sm">
													<RadioGroup.Label
														as="p"
														className={`font-medium  ${checked ? 'text-white' : 'text-gray-900'
															}`}
													>
														{chat.name}
													</RadioGroup.Label>
													<RadioGroup.Description
														as="span"
														className={`inline ${checked ? 'text-sky-100' : 'text-gray-500'
															}`}
													>
														<p>{chat.msg}</p>
													</RadioGroup.Description>
												</div>
											</div>
										</div>
									</>
								)}
							</RadioGroup.Option>
						))}
					</div>
				</RadioGroup>
			</div>
		</div>
	)
}