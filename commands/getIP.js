const { SlashCommandBuilder } = require('discord.js');
const { version } = require('node:os');
const { Resolver } = require('node:dns').promises;
const resolver = new Resolver();
resolver.setServers(['1.1.1.1']);

module.exports = {
	data: new SlashCommandBuilder()
		.setName('get_ip')
		.setDescription('Gets current IP of a server!')
		.addStringOption(option =>
			option.setName('domain')
				.setDescription('The domain to resolve')
				.setRequired(true)),
	async execute(interaction) {
		const domain = interaction.options.getString('domain') 
		if (!domain) {
			return interaction.reply({content: "Error, no domain specified", ephemeral: true});
		}

		await interaction.deferReply();
		let ips = await resolver.resolve4(domain).then((addresses) => {
			return(addresses)
		});
		ip_str = ""
		ips.map((ip) => {
			ip_str += "\n" + ip;
		})
		return interaction.editReply(`**${domain}:** ${ip_str}`);

	},
};
