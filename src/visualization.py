import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

if __name__ == '__main__':

    data_name = input('Data name: ')
    df = pd.read_csv('../output/' + data_name + '.csv')

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Average Degree')
    ax.set_ylim([0, df['prod'].max()+1])
    ax.set_title('Estimate average degree on ' + data_name + ' data')
    plt.plot(df['p'], df['avgd'], '--k', label='Average Degree of G')
    plt.plot(df['p'], df['orid'], '-go', label='Smooth Estimator')
    plt.plot(df['p'], df['prod'], '-ro', label='Modified Estimator')

    plt.plot(df['p'], df['avgd'] * (1-df['p']), '.-', label='(1-p) Average Degree')
    plt.plot(df['p'], df['avgd*'], '--', label='Average Degree of G*')

    plt.legend()
    plt.savefig(data_name+'_avgd.jpg', dpi=450)

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Average Clustering Coefficient')
    ax.set_title('Estimate average clustering coefficient on ' +
                 data_name + ' data')
    ax.set_ylim([0, df['proavgc'].max()+0.1])
    plt.plot(df['p'], df['avgc'], '--k', label='Average Clustering Coefficient of G')
    plt.plot(df['p'], df['oriavgc'], '-go', label='Original Estimator')
    plt.plot(df['p'], df['proavgc'], '-ro', label='Proposed Estimator')

    plt.plot(df['p'], df['avgc*'], '--', label='Average Clustering Coefficient of G*')
    plt.legend()
    plt.savefig(data_name+'_avgc.jpg', dpi=450)


    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Size')
    ax.set_title('Estimate size on ' +
                 data_name + ' data')
    ax.set_ylim([0, df['size'].max()*1.1])
    plt.plot(df['p'], df['size'], '--k', label='Size of G')
    plt.plot(df['p'], df['oris'], '-go', label='Original Estimator')
    plt.plot(df['p'], df['pros'], '-ro', label='Proposed Estimator')

    plt.plot(df['p'], df['size*'], '--', label='Size of G*')
    plt.plot(df['p'], df['size'] * (1-df['p']), '--', label='(1-p) size of G')
    plt.legend()
    plt.savefig(data_name+'_size.jpg', dpi=450)

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Size')
    ax.set_title('Estimate global clustering coefficient on ' +
                 data_name + ' data')
    ax.set_ylim([0, df['origloc'].max()*2])
    plt.plot(df['p'], df['globalc'], '--k', label='Global Clustering Coefficient of G')
    plt.plot(df['p'], df['origloc'], '-go', label='Original Estimator')

    plt.legend()
    plt.savefig(data_name+'_gc.jpg', dpi=450)

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Error')
    ax.set_title('Error of average clustering coefficient on ' +
                 data_name + ' data')
    plt.plot(df['p'], (1-df['oriavgc']/df['avgc'])**2,
             '-go', label='Original Estimator Error')
    plt.plot(df['p'], (1-df['proavgc']/df['avgc'])**2,
             '-ro', label='Proposed Estimator Error')
    plt.legend()
    plt.savefig(data_name+'_avgcerror.jpg', dpi=450)

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Error')
    ax.set_title('Error of estimate average degree on ' + data_name + ' data')
    plt.plot(df['p'], (1-df['orid']/df['avgd'])**2,
             '-go', label='Smooth Estimator Error')
    plt.plot(df['p'], (1-df['prod']/df['avgd'])**2,
             '-ro', label='Proposed Estimator Error')
    plt.legend()
    plt.savefig(data_name+'_avgderror.jpg', dpi=450)

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Error')
    ax.set_title('Error of estimate size on ' + data_name + ' data')
    plt.plot(df['p'], (1-df['oris']/df['size'])**2,
             '-go', label='Smooth Estimator Error')
    plt.plot(df['p'], (1-df['pros']/df['size'])**2,
             '-ro', label='Proposed Estimator Error')
    plt.legend()
    plt.savefig(data_name+'_sizeerror.jpg', dpi=450)

    fig, ax = plt.subplots()

    ax.set_xlabel('Private probability p')
    ax.set_ylabel('Error')
    ax.set_title('Error of global clustering coefficient on ' + data_name + ' data')
    plt.plot(df['p'], (1-df['origloc']/df['globalc'])**2,
             '-go', label='Estimator Error')
    plt.legend()
    plt.savefig(data_name+'_gcerror.jpg', dpi=450)

